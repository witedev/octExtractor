mod headers;
mod image_processing;
mod utils;
mod parser;

use clap::{Arg, Command};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use rayon::prelude::*;
use image_processing::{read_fundus_image, read_img_jpeg, read_grayscale_image, read_thumbnail};
use utils::{get_list_of_file_chunks, read_all_metadata, empty_directory};
use env_logger;
use image::ImageFormat;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("OCT Extractor")
        .version("1.0")
        .author("Jesus Blanco - witeDev")
        .about("Extracts images from OCT files")
        .arg(Arg::new("filepath")
            .help("The path to the input file")
            .required(true)
            .index(1))
        .arg(Arg::new("output_format")
            .short('e')
            .long("extension")
            .help("The output image format")
            .required(true)
            .value_parser(["bmp", "jpg", "png", "tiff"]))
        .arg(Arg::new("output_dir")
            .short('o')
            .long("output")
            .help("The output directory")
            .default_value("extraction"))
        .get_matches();

    let filepath = matches.get_one::<String>("filepath").expect("filepath is required");
    let output_format_str = matches.get_one::<String>("output_format").expect("output format is required");
    let binding = "extraction".to_string();
    let output_dir = matches.get_one::<String>("output_dir").unwrap_or(&binding);

    let output_format = match output_format_str.as_str() {
        "bmp" => Some(ImageFormat::Bmp),
        "jpg" => Some(ImageFormat::Jpeg),
        "png" => Some(ImageFormat::Png),
        "tiff" => Some(ImageFormat::Tiff),
        _ => unreachable!(), // Clap ensures only valid values
    };

    let (chunk_dict, _header) = get_list_of_file_chunks(filepath, true)?;
    // Inicializa el registrador
    env_logger::init();

    // Crear y vaciar el directorio de salida
    empty_directory(output_dir)?;

    // Crear las subcarpetas necesarias
    let subdirs = ["oct", "fundus", "grayscale", "thumbnail", "metadata"];
    for subdir in &subdirs {
        fs::create_dir_all(format!("{}/{}", output_dir, subdir))?;
    }

    let metadata = read_all_metadata(filepath, &chunk_dict, true)?;

    let metadata_json = serde_json::to_string_pretty(&metadata)?;
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(format!("{}/metadata/metadata.json", output_dir))?;
    file.write_all(metadata_json.as_bytes())?;

    let tasks: Vec<Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send>> = vec![
        Box::new(|| read_img_jpeg(filepath, &chunk_dict, output_format.clone(), output_dir)),
        Box::new(|| read_fundus_image(filepath, &chunk_dict, output_format.clone(), output_dir)),
        Box::new(|| read_grayscale_image(filepath, &chunk_dict, output_format.clone(), output_dir)),
        Box::new(|| read_thumbnail(filepath, &chunk_dict, output_dir)),
    ];

    tasks.into_par_iter().for_each(|task| {
        if let Err(e) = task() {
            eprintln!("Error: {:?}", e);
        }
    });

    Ok(())
}
