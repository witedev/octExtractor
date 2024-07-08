use image::{DynamicImage, ImageBuffer, ImageFormat, Luma, Rgba};
use jpeg2k::Image as Jpeg2kImage;
use log::{error, info};
use rayon::prelude::*;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::collections::HashMap;
use crate::fda::headers::ThumbnailHeader;

const J2K_SOI: &[u8] = &[0xFF, 0x4F, 0xFF, 0x51];

fn save_image_to_file(image: &DynamicImage, path: &str, format: Option<ImageFormat>) -> Result<(), Box<dyn Error>> {
    match format {
        Some(format) => {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?;
            if format == ImageFormat::Jpeg && image.color().has_alpha() {
                // Convert Rgba8 to Rgb8 for JPEG
                let rgb_image = image.to_rgb8();
                DynamicImage::ImageRgb8(rgb_image).write_to(&mut file, format)?;
            } else {
                image.write_to(&mut file, format)?;
            }
        }
        None => {
            // Save as .j2k
            let image_data = image.clone().into_bytes();
            save_j2k_file(&image_data, path)?;
        }
    }
    Ok(())
}

fn convert_bgr_to_rgb(image_data: &mut [u8]) {
    for chunk in image_data.chunks_exact_mut(4) {
        chunk.swap(0, 2); // Swap B and R
    }
}

fn scale_16bit_to_8bit(data: &[u8]) -> Vec<u8> {
    data.chunks_exact(2)
        .map(|chunk| (u16::from_be_bytes([chunk[0], chunk[1]]) >> 8) as u8)
        .collect()
}

fn save_j2k_to_format(j2k_data: &[u8], base_path: &str, format: Option<ImageFormat>, is_bgr: bool, is_greyscale_16bit: bool) -> Result<(), Box<dyn Error>> {
    if format.is_none() {
        // Directly save the raw J2K data
        let j2k_path = format!("{}.j2k", base_path);
        return Ok(save_j2k_file(j2k_data, &j2k_path)?);
    }

    let jp2_image = Jpeg2kImage::from_bytes(j2k_data)?;
    let width = jp2_image.width();
    let height = jp2_image.height();

    let mut image_data = jp2_image.get_pixels(Some(255))?;
    if is_bgr {
        convert_bgr_to_rgb(&mut image_data.data);
    }

    let img = if jp2_image.num_components() == 1 {
        if is_greyscale_16bit && image_data.data.len() == width as usize * height as usize * 2 {
            let luma_data_8bit = scale_16bit_to_8bit(&image_data.data);
            let luma_buffer: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, luma_data_8bit)
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to create Luma ImageBuffer"))?;
            DynamicImage::ImageLuma8(luma_buffer)
        } else {
            let luma_buffer: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, image_data.data.clone())
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to create Luma ImageBuffer"))?;
            DynamicImage::ImageLuma8(luma_buffer)
        }
    } else {
        let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, image_data.data.clone())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to create ImageBuffer"))?;
        DynamicImage::ImageRgba8(buffer)
    };

    let extension = if let Some(format) = format {
        format.extensions_str()[0]
    } else {
        "j2k"
    };

    save_image_to_file(&img, &format!("{}.{}", base_path, extension), format)?;
    Ok(())
}

fn extract_images_from_chunk_parallel(data: &[u8], output_dir: &str, prefix: &str, format: Option<ImageFormat>, is_bgr: bool, is_greyscale_16bit: bool) -> Result<(), Box<dyn Error>> {
    let mut start = 0;
    let mut positions = vec![];

    while let Some(pos) = data[start..].windows(4).position(|window| window == J2K_SOI) {
        positions.push(start + pos);
        start += pos + J2K_SOI.len();
    }

    positions.push(data.len()); // Add the end of the data as the final position

    positions.par_windows(2).enumerate().for_each(|(image_count, window)| {
        let image_start = window[0];
        let image_end = window[1];
        let image_data = &data[image_start..image_end];
        let base_path = format!("{}/{}_{}", output_dir, prefix, image_count);

        if let Err(e) = save_j2k_to_format(image_data, &base_path, format, is_bgr, is_greyscale_16bit) {
            error!("Failed to save image formats for {}_{}: {}", prefix, image_count, e);
        }
    });

    Ok(())
}

pub fn read_fundus_image(filepath: &str, chunk_dict: &HashMap<String, (u64, u32)>, format: Option<ImageFormat>, output_dir: &str) -> Result<(), Box<dyn Error>> {
    if let Some(&(chunk_location, chunk_size)) = chunk_dict.get("@IMG_FUNDUS") {
        let mut raw_image = vec![0; chunk_size as usize];
        let mut file = File::open(filepath)?;
        file.seek(SeekFrom::Start(chunk_location))?;
        file.read_exact(&mut raw_image)?;

        extract_images_from_chunk_parallel(&raw_image, &format!("{}/fundus", output_dir), "fundus", format, true, false)?;
        Ok(())
    } else {
        info!("@IMG_FUNDUS is not in chunk list, skipping.");
        Err("Chunk @IMG_FUNDUS not found".into())
    }
}

pub fn read_img_jpeg(filepath: &str, chunk_dict: &HashMap<String, (u64, u32)>, format: Option<ImageFormat>, output_dir: &str) -> Result<(), Box<dyn Error>> {
    if let Some(&(chunk_location, chunk_size)) = chunk_dict.get("@IMG_JPEG") {
        let mut raw_image = vec![0; chunk_size as usize];
        let mut file = File::open(filepath)?;
        file.seek(SeekFrom::Start(chunk_location))?;
        file.read_exact(&mut raw_image)?;

        extract_images_from_chunk_parallel(&raw_image, &format!("{}/oct", output_dir), "bscan", format, false, true)?;
        Ok(())
    } else {
        info!("@IMG_JPEG is not in chunk list, skipping.");
        Err("Chunk @IMG_JPEG not found".into())
    }
}

pub fn read_grayscale_image(filepath: &str, chunk_dict: &HashMap<String, (u64, u32)>, format: Option<ImageFormat>, output_dir: &str) -> Result<(), Box<dyn Error>> {
    if let Some(&(chunk_location, chunk_size)) = chunk_dict.get("@IMG_TRC_02") {
        let mut raw_image = vec![0; chunk_size as usize];
        let mut file = File::open(filepath)?;
        file.seek(SeekFrom::Start(chunk_location))?;
        file.read_exact(&mut raw_image)?;

        extract_images_from_chunk_parallel(&raw_image, &format!("{}/grayscale", output_dir), "grayscale_fundus", format, false, false)?;
        Ok(())
    } else {
        info!("@IMG_TRC_02 is not in chunk list, skipping.");
        Err("Chunk @IMG_TRC_02 not found".into())
    }
}

pub fn read_thumbnail(filepath: &str, chunk_dict: &HashMap<String, (u64, u32)>, output_dir: &str) -> Result<(), Box<dyn Error>> {
    if let Some(&(chunk_location, _chunk_size)) = chunk_dict.get("@THUMBNAIL") {
        let mut file = File::open(filepath)?;
        file.seek(SeekFrom::Start(chunk_location))?;

        let header = ThumbnailHeader::from_reader(&mut file)?;
        let image = image::load_from_memory_with_format(&header.img, ImageFormat::Bmp)?;
        let thumbnail_path = format!("{}/thumbnail/thumbnail.bmp", output_dir);
        image.save(&thumbnail_path)?;

        Ok(())
    } else {
        info!("@THUMBNAIL is not in chunk list, skipping.");
        Err("Chunk @THUMBNAIL not found".into())
    }
}

fn save_j2k_file(j2k_data: &[u8], path: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(j2k_data)?;
    Ok(())
}
