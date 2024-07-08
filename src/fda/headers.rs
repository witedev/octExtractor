use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};

// Helper function to read padded strings
fn read_padded_string<R: Read>(reader: &mut R, len: usize) -> Result<String, io::Error> {
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).replace('\u{0000}', "").to_string())
}



#[derive(Debug)]
pub struct Header {
    pub file_code: String,
    pub file_type: String,
    pub major_ver: u32,
    pub minor_ver: u32,
}

impl Header {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let file_code = read_padded_string(reader, 4)?;
        let file_type = read_padded_string(reader, 3)?;
        let major_ver = reader.read_u32::<LittleEndian>()?;
        let minor_ver = reader.read_u32::<LittleEndian>()?;
        Ok(Header {
            file_code,
            file_type,
            major_ver,
            minor_ver,
        })
    }
}

#[derive(Debug)]
pub struct CaptureInfo02Header {
    pub eye: String,
    pub scan_mode: u8,
    pub session_id: u32,
    pub label: String,
    pub cap_date: String,
}

impl CaptureInfo02Header {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let eye = match reader.read_u8()? {
            1 => "LEFT".to_string(),
            0 => "RIGHT".to_string(),
            _ => "UNKNOWN".to_string(),
        };
        let scan_mode = reader.read_u8()?;
        let session_id = reader.read_u32::<LittleEndian>()?;
        let label = read_padded_string(reader, 100)?;

        let mut cap_date = [0u16; 6];
        for i in 0..6 {
            cap_date[i] = reader.read_u16::<LittleEndian>()?;
        }
        let cap_date = format!(
            "{}-{}-{} {}:{}:{}",
            cap_date[0], cap_date[1], cap_date[2], cap_date[3], cap_date[4], cap_date[5]
        );

        Ok(CaptureInfo02Header {
            eye,
            scan_mode,
            session_id,
            label,
            cap_date,
        })
    }
}

#[derive(Debug)]
pub struct HwInfo03Header {
    pub model_name: String,
    pub serial_number: String,
    pub spect_sn: String,
    pub rom_ver: String,
    pub unknown: String,
    pub eq_calib_year: u16,
    pub eq_calib_month: u16,
    pub eq_calib_day: u16,
    pub eq_calib_hour: u16,
    pub eq_calib_minute: u16,
    pub spect_calib_year: u16,
    pub spect_calib_month: u16,
    pub spect_calib_day: u16,
    pub spect_calib_hour: u16,
    pub spect_calib_minute: u16,
}

impl HwInfo03Header {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(HwInfo03Header {
            model_name: read_padded_string(reader, 16)?.replace("\u{0}", "").to_string(),
            serial_number: read_padded_string(reader, 16)?,
            spect_sn: read_padded_string(reader, 16)?,
            rom_ver: read_padded_string(reader, 16)?,
            unknown: read_padded_string(reader, 16)?,
            eq_calib_year: reader.read_u16::<LittleEndian>()?,
            eq_calib_month: reader.read_u16::<LittleEndian>()?,
            eq_calib_day: reader.read_u16::<LittleEndian>()?,
            eq_calib_hour: reader.read_u16::<LittleEndian>()?,
            eq_calib_minute: reader.read_u16::<LittleEndian>()?,
            spect_calib_year: reader.read_u16::<LittleEndian>()?,
            spect_calib_month: reader.read_u16::<LittleEndian>()?,
            spect_calib_day: reader.read_u16::<LittleEndian>()?,
            spect_calib_hour: reader.read_u16::<LittleEndian>()?,
            spect_calib_minute: reader.read_u16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct PatientInfo02Header {
    pub patient_id: String,
    pub given_name: String,
    pub surname: String,
    pub birth_date_valid: bool,
    pub birth_year: u16,
    pub birth_month: u16,
    pub birth_day: u16,
    pub extra_data: Vec<u8>,
}

impl PatientInfo02Header {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let patient_id = read_padded_string(reader, 32)?;
        let given_name = read_padded_string(reader, 32)?;
        let surname = read_padded_string(reader, 32)?;
        
        let mut zeros = [0u8; 8];
        reader.read_exact(&mut zeros)?;

        let birth_date_valid = match reader.read_u8()? {
            1 => true,
            _ => false,
        };
        
        let birth_year = reader.read_u16::<LittleEndian>()?;
        let birth_month = reader.read_u16::<LittleEndian>()?;
        let birth_day = reader.read_u16::<LittleEndian>()?;

        let mut extra_data = vec![0; 504];
        reader.read_exact(&mut extra_data)?;

        Ok(PatientInfo02Header {
            patient_id,
            given_name,
            surname,
            birth_date_valid,
            birth_year,
            birth_month,
            birth_day,
            extra_data,
        })
    }
}

#[derive(Debug)]
pub struct PatientInfo03Header {
    pub patient_id: String,
    pub given_name: String,
    pub surname: String,
    pub sex: String,
    pub birth_date: String,
}

impl PatientInfo03Header {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let patient_id = read_padded_string(reader, 32)?.replace("\u{0}", "").to_string();
        let given_name = read_padded_string(reader, 32)?.replace("\u{0}", "").to_string();
        let surname = read_padded_string(reader, 32)?.replace("\u{0}", "").to_string();

        let sex = match reader.read_u8()? {
            1 => "M".to_string(),
            2 => "F".to_string(),
            3 => "O".to_string(),
            _ => "Unknown".to_string(),
        };

        let mut birth_date_arr = [0u16; 3];
        for i in 0..3 {
            birth_date_arr[i] = reader.read_u16::<LittleEndian>()?;
        }
        let birth_date = format!("{}, {}, {}", birth_date_arr[0], birth_date_arr[1], birth_date_arr[2]);

        Ok(PatientInfo03Header {
            patient_id,
            given_name,
            surname,
            sex,
            birth_date,
        })
    }
}

#[derive(Debug)]
pub struct ImgJpegHeader {
    pub scan_mode: u8,
    pub unknown1: u32,
    pub unknown2: u32,
    pub width: u32,
    pub height: u32,
    pub number_slices: u32,
    pub unknown3: u32,
}

impl ImgJpegHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(ImgJpegHeader {
            scan_mode: reader.read_u8()?,
            unknown1: reader.read_u32::<LittleEndian>()?,
            unknown2: reader.read_u32::<LittleEndian>()?,
            width: reader.read_u32::<LittleEndian>()?,
            height: reader.read_u32::<LittleEndian>()?,
            number_slices: reader.read_u32::<LittleEndian>()?,
            unknown3: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct ImgMotComp03Header {
    pub scan_mode: u8,
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u32,
    pub number_slices: u32,
    pub format: u8,
    pub size: u32,
}

impl ImgMotComp03Header {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(ImgMotComp03Header {
            scan_mode: reader.read_u8()?,
            width: reader.read_u32::<LittleEndian>()?,
            height: reader.read_u32::<LittleEndian>()?,
            bits_per_pixel: reader.read_u32::<LittleEndian>()?,
            number_slices: reader.read_u32::<LittleEndian>()?,
            format: reader.read_u8()?,
            size: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct ResultCorneaCurveHeader {
    pub id: [u8; 20],
    pub width: u32,
    pub height: u32,
    pub version: [u8; 32],
}

impl ResultCorneaCurveHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut id = [0u8; 20];
        reader.read_exact(&mut id)?;
        let width = reader.read_u32::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;
        let mut version = [0u8; 32];
        reader.read_exact(&mut version)?;
        Ok(ResultCorneaCurveHeader {
            id,
            width,
            height,
            version,
        })
    }
}

#[derive(Debug)]
pub struct FdaFileInfoHeader {
    pub field_0x2: u32,
    pub field_0x3e8: u32,
    pub version: String,
}

impl FdaFileInfoHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(FdaFileInfoHeader {
            field_0x2: reader.read_u32::<LittleEndian>()?,
            field_0x3e8: reader.read_u32::<LittleEndian>()?,
            version: read_padded_string(reader, 32)?,
        })
    }
}

#[derive(Debug)]
pub struct ResultCorneaThicknessHeader {
    pub version: [u8; 32],
    pub id: [u8; 20],
    pub width: u32,
    pub height: u32,
}

impl ResultCorneaThicknessHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut version = [0u8; 32];
        reader.read_exact(&mut version)?;
        let mut id = [0u8; 20];
        reader.read_exact(&mut id)?;
        let width = reader.read_u32::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;
        Ok(ResultCorneaThicknessHeader {
            version,
            id,
            width,
            height,
        })
    }
}

#[derive(Debug)]
pub struct ContourInfoHeader {
    pub id: String,
    pub method: u8,
    pub format: u8,
    pub width: u32,
    pub height: u32,
    pub size: u32,
}

impl ContourInfoHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(ContourInfoHeader {
            id: read_padded_string(reader, 20)?,
            method: reader.read_u8()?,
            format: reader.read_u8()?,
            width: reader.read_u32::<LittleEndian>()?,
            height: reader.read_u32::<LittleEndian>()?,
            size: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct AlignInfoHeader {
    pub unlabeled_1: u8,
    pub unlabeled_2: u8,
    pub w: u32,
    pub n_size: u32,
    pub aligndata: Option<Vec<u16>>,
    pub keyframe_1: u32,
    pub keyframe_2: u32,
    pub unlabeled_3: u32,
    pub unlabeled_4: u32,
}

impl AlignInfoHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
 

        let unlabeled_1 = reader.read_u8()?;
        let unlabeled_2 = reader.read_u8()?;
       

        let w = reader.read_u32::<LittleEndian>()?;
        let n_size = reader.read_u32::<LittleEndian>()?;


        let aligndata = if n_size > 0 && w < 10000 {
            let size = (w * 2) as usize;
            if size > 0 && size < 1000000 {
                // println!("  Reading aligndata of size: {}", size);
                let mut data = vec![0u16; size];
                for i in 0..size {
                    data[i] = reader.read_u16::<LittleEndian>()?;
                }
                Some(data)
            } else {
                // println!("  Invalid aligndata size: {}", size);
                None
            }
        } else {
            // println!("  Skipping aligndata due to invalid n_size or w");
            None
        };

        let keyframe_1 = reader.read_u32::<LittleEndian>()?;
        let keyframe_2 = reader.read_u32::<LittleEndian>()?;
        let unlabeled_3 = reader.read_u32::<LittleEndian>()?;
        let unlabeled_4 = reader.read_u32::<LittleEndian>()?;

     

        Ok(AlignInfoHeader {
            unlabeled_1,
            unlabeled_2,
            w,
            n_size,
            aligndata,
            keyframe_1,
            keyframe_2,
            unlabeled_3,
            unlabeled_4,
        })
    }
}

#[derive(Debug)]
pub struct ParamScan04Header {
    pub fixation: u32,
    pub mirror_pos: u32,
    pub polar: u32,
    pub x_dimension_mm: f64,
    pub y_dimension_mm: f64,
    pub z_resolution_um: f64,
    pub comp_eff_2: f64,
    pub comp_eff_3: f64,
    pub base_pos: u8,
    pub used_calib_data: u8,
}

impl ParamScan04Header {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(ParamScan04Header {
            fixation: reader.read_u32::<LittleEndian>()?,
            mirror_pos: reader.read_u32::<LittleEndian>()?,
            polar: reader.read_u32::<LittleEndian>()?,
            x_dimension_mm: reader.read_f64::<LittleEndian>()?,
            y_dimension_mm: reader.read_f64::<LittleEndian>()?,
            z_resolution_um: reader.read_f64::<LittleEndian>()?,
            comp_eff_2: reader.read_f64::<LittleEndian>()?,
            comp_eff_3: reader.read_f64::<LittleEndian>()?,
            base_pos: reader.read_u8()?,
            used_calib_data: reader.read_u8()?,
        })
    }
}

#[derive(Debug)]
pub struct MainModuleInfoHeader {
    pub software_name: String,
    pub file_version_1: u16,
    pub file_version_2: u16,
    pub file_version_3: u16,
    pub file_version_4: u16,
    pub string: String,
}

impl MainModuleInfoHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(MainModuleInfoHeader {
            software_name: read_padded_string(reader, 128)?,
            file_version_1: reader.read_u16::<LittleEndian>()?,
            file_version_2: reader.read_u16::<LittleEndian>()?,
            file_version_3: reader.read_u16::<LittleEndian>()?,
            file_version_4: reader.read_u16::<LittleEndian>()?,
            string: read_padded_string(reader, 128)?,
        })
    }
}

#[derive(Debug)]
pub struct ThumbnailHeader {
    pub size: u32,
    pub img: Vec<u8>,
}

impl ThumbnailHeader {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let size = reader.read_u32::<LittleEndian>()?;
        let mut img = vec![0u8; size as usize];
        reader.read_exact(&mut img)?;
        Ok(ThumbnailHeader { size, img })
    }
}


#[derive(Debug)]
pub struct ContourMaskInfoHeader {
    pub empty: bool,
}

impl ContourMaskInfoHeader {
    pub fn parse<R: Read>(_reader: &mut R) -> io::Result<Self> {
        Ok(ContourMaskInfoHeader { empty: true })
    }
}

#[derive(Debug)]
pub struct TopQExtInfoHeader {
    pub empty: bool,
}

impl TopQExtInfoHeader {
    pub fn parse<R: Read>(_reader: &mut R) -> io::Result<Self> {
        Ok(TopQExtInfoHeader { empty: true })
    }
}

#[derive(Debug)]
pub struct EffectiveScanRangeHeader {
    pub fundus_bounding_box: [u32; 4],
    pub trc_bounding_box: [u32; 4],
}

impl EffectiveScanRangeHeader {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut fundus_bounding_box = [0u32; 4];
        for i in 0..4 {
            fundus_bounding_box[i] = reader.read_u32::<LittleEndian>()?;
        }
        let mut trc_bounding_box = [0u32; 4];
        for i in 0..4 {
            trc_bounding_box[i] = reader.read_u32::<LittleEndian>()?;
        }
        Ok(EffectiveScanRangeHeader {
            fundus_bounding_box,
            trc_bounding_box,
        })
    }
}

#[derive(Debug)]
pub struct FastQ2InfoHeader {
    pub various_quality_statistics: [f32; 6],
}

impl FastQ2InfoHeader {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut various_quality_statistics = [0f32; 6];
        for i in 0..6 {
            various_quality_statistics[i] = reader.read_f32::<LittleEndian>()?;
        }
        Ok(FastQ2InfoHeader {
            various_quality_statistics,
        })
    }
}

#[derive(Debug)]
pub struct ParamObs02Header {
    pub values: [u16; 3],
    pub camera_model: String,
    pub jpeg_quality: String,
    pub color_temperature: String,
    pub color_temperature_value: u16,
}

impl ParamObs02Header {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let values = [
            reader.read_u16::<LittleEndian>()?,
            reader.read_u16::<LittleEndian>()?,
            reader.read_u16::<LittleEndian>()?,
        ];
        let camera_model = read_padded_string(reader, 12)?;
        let jpeg_quality = read_padded_string(reader, 24)?;
        let _unknown1 = reader.read_u16::<LittleEndian>()?;
        let _unknown2 = reader.read_u16::<LittleEndian>()?;
        let _unknown3 = reader.read_u16::<LittleEndian>()?;
        let color_temperature = read_padded_string(reader, 24)?;
        let color_temperature_value = reader.read_u16::<LittleEndian>()?;
        let mut _zeros = [0u8; 12];
        reader.read_exact(&mut _zeros)?;
        let _unknown4 = reader.read_f32::<LittleEndian>()?;
        Ok(ParamObs02Header {
            values,
            camera_model,
            jpeg_quality,
            color_temperature,
            color_temperature_value,
        })
    }
}

#[derive(Debug)]
pub struct RegistInfoHeader {
    pub u8_value: u8,
    pub u32_values_1: [u32; 2],
    pub bounding_box_fundus: [u32; 4],
    pub u8_string: String,
    pub bounding_box_trc: [u32; 4],
    pub f64_values: [f64; 4],
    pub zeros: [u8; 48],
}

impl RegistInfoHeader {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let u8_value = reader.read_u8()?;
        let u32_values_1 = [
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
        ];
        let bounding_box_fundus = [
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
        ];
        let u8_string = read_padded_string(reader, 32)?;
        let bounding_box_trc = [
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
        ];
        let f64_values = [
            reader.read_f64::<LittleEndian>()?,
            reader.read_f64::<LittleEndian>()?,
            reader.read_f64::<LittleEndian>()?,
            reader.read_f64::<LittleEndian>()?,
        ];
        let mut zeros = [0u8; 48];
        reader.read_exact(&mut zeros)?;

        Ok(RegistInfoHeader {
            u8_value,
            u32_values_1,
            bounding_box_fundus,
            u8_string,
            bounding_box_trc,
            f64_values,
            zeros,
        })
    }
}

#[derive(Debug)]
pub struct GlaLittmann01Header {
    pub u32_values: [u32; 11],
    pub u32_value_1: u32,
    pub u32_value_2: u32,
}

impl GlaLittmann01Header {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut u32_values = [0u32; 11];
        for i in 0..11 {
            u32_values[i] = reader.read_u32::<LittleEndian>()?;
        }
        let u32_value_1 = reader.read_u32::<LittleEndian>()?;
        let u32_value_2 = reader.read_u32::<LittleEndian>()?;
        Ok(GlaLittmann01Header {
            u32_values,
            u32_value_1,
            u32_value_2,
        })
    }
}

#[derive(Debug)]
pub struct ImgEnFaceHeader {
    pub empty: bool,
}

impl ImgEnFaceHeader {
    pub fn parse<R: Read>(_reader: &mut R) -> io::Result<Self> {
        Ok(ImgEnFaceHeader { empty: true })
    }
}

#[derive(Debug)]
pub struct ReportInfoHeader {
    pub zeros: [u8; 7],
}

impl ReportInfoHeader {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut zeros = [0u8; 7];
        reader.read_exact(&mut zeros)?;
        Ok(ReportInfoHeader { zeros })
    }
}
