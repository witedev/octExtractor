use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Seek};

use crate::fda::headers::*;

pub fn parse_chunk<R: Read + Seek>(reader: &mut R, chunk_name: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut chunk_info = HashMap::new();
    let chunk_name_clean = chunk_name.trim_start_matches('@').to_lowercase();

    match chunk_name_clean.as_str() {
        "patient_info_02" => {
            let patient_info = PatientInfo02Header::parse(reader)?;
            chunk_info.insert("patient_id".to_string(), patient_info.patient_id);
            chunk_info.insert("given_name".to_string(), patient_info.given_name);
            chunk_info.insert("surname".to_string(), patient_info.surname);
            chunk_info.insert("birth_date_valid".to_string(), patient_info.birth_date_valid.to_string());
            chunk_info.insert("birth_year".to_string(), patient_info.birth_year.to_string());
            chunk_info.insert("birth_month".to_string(), patient_info.birth_month.to_string());
            chunk_info.insert("birth_day".to_string(), patient_info.birth_day.to_string());
            chunk_info.insert("extra_data".to_string(), format!("{:?}", patient_info.extra_data));
        }
        "header" => {
            let header = Header::parse(reader)?;
            chunk_info.insert("file_code".to_string(), header.file_code);
            chunk_info.insert("file_type".to_string(), header.file_type);
            chunk_info.insert("major_ver".to_string(), header.major_ver.to_string());
            chunk_info.insert("minor_ver".to_string(), header.minor_ver.to_string());
        }
        "capture_info_02" => {
            let capture_info = CaptureInfo02Header::parse(reader)?;
            chunk_info.insert("eye".to_string(), capture_info.eye.to_string());
            chunk_info.insert("scan_mode".to_string(), capture_info.scan_mode.to_string());
            chunk_info.insert("session_id".to_string(), capture_info.session_id.to_string());
            chunk_info.insert("label".to_string(), capture_info.label);
            chunk_info.insert("cap_date".to_string(), format!("{:?}", capture_info.cap_date));
        }
        "hw_info_03" => {
            let hw_info = HwInfo03Header::parse(reader)?;
            chunk_info.insert("model_name".to_string(), hw_info.model_name);
            chunk_info.insert("serial_number".to_string(), hw_info.serial_number);
            chunk_info.insert("spect_sn".to_string(), hw_info.spect_sn);
            chunk_info.insert("rom_ver".to_string(), hw_info.rom_ver);
            chunk_info.insert("unknown".to_string(), hw_info.unknown);
            chunk_info.insert("eq_calib_year".to_string(), hw_info.eq_calib_year.to_string());
            chunk_info.insert("eq_calib_month".to_string(), hw_info.eq_calib_month.to_string());
            chunk_info.insert("eq_calib_day".to_string(), hw_info.eq_calib_day.to_string());
            chunk_info.insert("eq_calib_hour".to_string(), hw_info.eq_calib_hour.to_string());
            chunk_info.insert("eq_calib_minute".to_string(), hw_info.eq_calib_minute.to_string());
            chunk_info.insert("spect_calib_year".to_string(), hw_info.spect_calib_year.to_string());
            chunk_info.insert("spect_calib_month".to_string(), hw_info.spect_calib_month.to_string());
            chunk_info.insert("spect_calib_day".to_string(), hw_info.spect_calib_day.to_string());
            chunk_info.insert("spect_calib_hour".to_string(), hw_info.spect_calib_hour.to_string());
            chunk_info.insert("spect_calib_minute".to_string(), hw_info.spect_calib_minute.to_string());
        }
        "patient_info_03" => {
            let patient_info = PatientInfo03Header::from_reader(reader)?;
            chunk_info.insert("patient_id".to_string(), patient_info.patient_id);
            chunk_info.insert("surname".to_string(), patient_info.surname);
            chunk_info.insert("given_name".to_string(), patient_info.given_name);
            chunk_info.insert("sex".to_string(), patient_info.sex);
            chunk_info.insert("birth_date".to_string(), patient_info.birth_date);
        }
        "img_jpeg" => {
            let header = ImgJpegHeader::from_reader(reader)?;
            chunk_info.insert("scan_mode".to_string(), header.scan_mode.to_string());
            chunk_info.insert("unknown1".to_string(), header.unknown1.to_string());
            chunk_info.insert("unknown2".to_string(), header.unknown2.to_string());
            chunk_info.insert("width".to_string(), header.width.to_string());
            chunk_info.insert("height".to_string(), header.height.to_string());
            chunk_info.insert("number_slices".to_string(), header.number_slices.to_string());
            chunk_info.insert("unknown3".to_string(), header.unknown3.to_string());
        }
        "img_mot_comp_03" => {
            let header = ImgMotComp03Header::from_reader(reader)?;
            chunk_info.insert("scan_mode".to_string(), header.scan_mode.to_string());
            chunk_info.insert("width".to_string(), header.width.to_string());
            chunk_info.insert("height".to_string(), header.height.to_string());
            chunk_info.insert("bits_per_pixel".to_string(), header.bits_per_pixel.to_string());
            chunk_info.insert("number_slices".to_string(), header.number_slices.to_string());
            chunk_info.insert("format".to_string(), header.format.to_string());
            chunk_info.insert("size".to_string(), header.size.to_string());
        }
        "fda_file_info" => {
            let header = FdaFileInfoHeader::from_reader(reader)?;
            chunk_info.insert("0x2".to_string(), header.field_0x2.to_string());
            chunk_info.insert("0x3e8".to_string(), header.field_0x3e8.to_string());
            chunk_info.insert("version".to_string(), header.version);
        }
        "contour_info" => {
            let header = ContourInfoHeader::from_reader(reader)?;
            chunk_info.insert("id".to_string(), header.id);
            chunk_info.insert("method".to_string(), header.method.to_string());
            chunk_info.insert("format".to_string(), header.format.to_string());
            chunk_info.insert("width".to_string(), header.width.to_string());
            chunk_info.insert("height".to_string(), header.height.to_string());
            chunk_info.insert("size".to_string(), header.size.to_string());
        }
        "align_info" => {
            println!("Parsing align_info");
            let header = AlignInfoHeader::from_reader(reader)?;
            println!("Parsed align_info header: {:?}", header);

            chunk_info.insert("unlabeled_1".to_string(), header.unlabeled_1.to_string());
            chunk_info.insert("unlabeled_2".to_string(), header.unlabeled_2.to_string());
            chunk_info.insert("w".to_string(), header.w.to_string());
            chunk_info.insert("n_size".to_string(), header.n_size.to_string());

            if let Some(aligndata) = header.aligndata {
                chunk_info.insert("aligndata".to_string(), format!("{:?}", aligndata));
            } else {
                chunk_info.insert("aligndata".to_string(), "None".to_string());
            }

            chunk_info.insert("keyframe_1".to_string(), header.keyframe_1.to_string());
            chunk_info.insert("keyframe_2".to_string(), header.keyframe_2.to_string());
            chunk_info.insert("unlabeled_3".to_string(), header.unlabeled_3.to_string());
            chunk_info.insert("unlabeled_4".to_string(), header.unlabeled_4.to_string());
        }
        "param_scan_04" => {
            let header = ParamScan04Header::from_reader(reader)?;
            chunk_info.insert("fixation".to_string(), header.fixation.to_string());
            chunk_info.insert("mirror_pos".to_string(), header.mirror_pos.to_string());
            chunk_info.insert("polar".to_string(), header.polar.to_string());
            chunk_info.insert("x_dimension_mm".to_string(), header.x_dimension_mm.to_string());
            chunk_info.insert("y_dimension_mm".to_string(), header.y_dimension_mm.to_string());
            chunk_info.insert("z_resolution_um".to_string(), header.z_resolution_um.to_string());
            chunk_info.insert("comp_eff_2".to_string(), header.comp_eff_2.to_string());
            chunk_info.insert("comp_eff_3".to_string(), header.comp_eff_3.to_string());
            chunk_info.insert("base_pos".to_string(), header.base_pos.to_string());
            chunk_info.insert("used_calib_data".to_string(), header.used_calib_data.to_string());
        }
        "result_cornea_curve" => {
            let header = ResultCorneaCurveHeader::from_reader(reader)?;
            chunk_info.insert("id".to_string(), format!("{:?}", header.id));
            chunk_info.insert("width".to_string(), header.width.to_string());
            chunk_info.insert("height".to_string(), header.height.to_string());
            chunk_info.insert("version".to_string(), format!("{:?}", header.version));
        }
        "result_cornea_thickness" => {
            let header = ResultCorneaThicknessHeader::from_reader(reader)?;
            chunk_info.insert("version".to_string(), format!("{:?}", header.version));
            chunk_info.insert("id".to_string(), format!("{:?}", header.id));
            chunk_info.insert("width".to_string(), header.width.to_string());
            chunk_info.insert("height".to_string(), header.height.to_string());
        }
        "main_module_info" => {
            let header = MainModuleInfoHeader::from_reader(reader)?;
            chunk_info.insert("software_name".to_string(), header.software_name);
            chunk_info.insert("file_version_1".to_string(), header.file_version_1.to_string());
            chunk_info.insert("file_version_2".to_string(), header.file_version_2.to_string());
            chunk_info.insert("file_version_3".to_string(), header.file_version_3.to_string());
            chunk_info.insert("file_version_4".to_string(), header.file_version_4.to_string());
            chunk_info.insert("string".to_string(), header.string);
        }
        "contour_mask_info" => {
            let header = ContourMaskInfoHeader::parse(reader)?;
            chunk_info.insert("empty".to_string(), header.empty.to_string());
        }
        "topqext_info" => {
            let header = TopQExtInfoHeader::parse(reader)?;
            chunk_info.insert("empty".to_string(), header.empty.to_string());
        }
        "effective_scan_range" => {
            let header = EffectiveScanRangeHeader::parse(reader)?;
            chunk_info.insert("fundus_bounding_box".to_string(), format!("{:?}", header.fundus_bounding_box));
            chunk_info.insert("trc_bounding_box".to_string(), format!("{:?}", header.trc_bounding_box));
        }
        "fast_q2_info" => {
            let header = FastQ2InfoHeader::parse(reader)?;
            chunk_info.insert("various_quality_statistics".to_string(), format!("{:?}", header.various_quality_statistics));
        }
        "param_obs_02" => {
            let header = ParamObs02Header::parse(reader)?;
            chunk_info.insert("values".to_string(), format!("{:?}", header.values));
            chunk_info.insert("camera_model".to_string(), header.camera_model);
            chunk_info.insert("jpeg_quality".to_string(), header.jpeg_quality);
            chunk_info.insert("color_temperature".to_string(), header.color_temperature);
            chunk_info.insert("color_temperature_value".to_string(), header.color_temperature_value.to_string());
        }
        "regist_info" => {
            let header = RegistInfoHeader::parse(reader)?;
            chunk_info.insert("u8_value".to_string(), header.u8_value.to_string());
            chunk_info.insert("u32_values_1".to_string(), format!("{:?}", header.u32_values_1));
            chunk_info.insert("bounding_box_fundus".to_string(), format!("{:?}", header.bounding_box_fundus));
            chunk_info.insert("u8_string".to_string(), header.u8_string);
            chunk_info.insert("bounding_box_trc".to_string(), format!("{:?}", header.bounding_box_trc));
            chunk_info.insert("f64_values".to_string(), format!("{:?}", header.f64_values));
            chunk_info.insert("zeros".to_string(), format!("{:?}", header.zeros));
        }
        "gla_littmann_01" => {
            let header = GlaLittmann01Header::parse(reader)?;
            chunk_info.insert("u32_values".to_string(), format!("{:?}", header.u32_values));
            chunk_info.insert("u32_value_1".to_string(), header.u32_value_1.to_string());
            chunk_info.insert("u32_value_2".to_string(), header.u32_value_2.to_string());
        }
        "img_en_face" => {
            let header = ImgEnFaceHeader::parse(reader)?;
            chunk_info.insert("empty".to_string(), header.empty.to_string());
        }
        "report_info" => {
            let header = ReportInfoHeader::parse(reader)?;
            chunk_info.insert("zeros".to_string(), format!("{:?}", header.zeros));
        }
        "thumbnail" =>{
            let header = ThumbnailHeader::from_reader(reader)?;
            chunk_info.insert("size".to_string(), format!("{:?}", header.size));
        }
        _ => {
            println!("Unhandled chunk: {}", chunk_name_clean);
        }
    }
    Ok(chunk_info)
}
