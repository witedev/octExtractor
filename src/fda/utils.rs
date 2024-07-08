use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use crate::fda::headers::Header;
use crate::fda::parser::parse_chunk; 

pub fn get_list_of_file_chunks(filepath: &str, printing: bool) -> io::Result<(HashMap<String, (u64, u32)>, Header)> {
    let mut chunk_dict: HashMap<String, (u64, u32)> = HashMap::new();
    let mut file = File::open(filepath)?;

    let header = Header::parse(&mut file)?;

    let mut eof = false;
    while !eof {
        let chunk_name_size = file.read_u8()? as usize;
        if chunk_name_size == 0 {
            eof = true;
        } else {
            let mut chunk_name = vec![0; chunk_name_size];
            file.read_exact(&mut chunk_name)?;
            let chunk_name = String::from_utf8(chunk_name).unwrap();
            let chunk_size = file.read_u32::<LittleEndian>()?;
            let chunk_location = file.seek(SeekFrom::Current(0))?;
            file.seek(SeekFrom::Current(chunk_size as i64))?;

            if !chunk_dict.contains_key(&chunk_name) {
                chunk_dict.insert(chunk_name, (chunk_location, chunk_size));
            }
        }
    }

    if printing {
        println!("File {} contains the following chunks:", filepath);
        for key in chunk_dict.keys() {
            println!("{}", key);
        }
        println!("");
    }

    Ok((chunk_dict, header))
}

pub fn read_any_info_and_make_dict(filepath: &str, chunk_name: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut file = File::open(filepath)?;
    let (chunk_location, chunk_size) = match get_list_of_file_chunks(filepath, false)?.0.get(chunk_name) {
        Some((location, size)) => (*location, *size),  
        None => return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Chunk not found"))),
    };
    file.seek(SeekFrom::Start(chunk_location))?;
    let mut raw = vec![0; chunk_size as usize];
    file.read_exact(&mut raw)?;

    let chunk_name_lowercase = chunk_name.to_lowercase();
    let mut cursor = io::Cursor::new(raw);
    let chunk_info = parse_chunk(&mut cursor, &chunk_name_lowercase)?;

    Ok(chunk_info)
}

pub fn read_all_metadata(filepath: &str, chunk_dict: &HashMap<String, (u64, u32)>, verbose: bool) -> io::Result<HashMap<String, HashMap<String, String>>> {
    let mut metadata = HashMap::new();
    for key in chunk_dict.keys() {
        if key == "@IMG_JPEG" || key == "@IMG_FUNDUS" || key == "@IMG_TRC_02" {
            continue;
        } 
        let json_key = key.split('@').last().unwrap_or("").to_uppercase();
        match read_any_info_and_make_dict(filepath, key) {
            Ok(info) => {
                metadata.insert(json_key, info);
            }
            Err(_) => {
                if verbose {
                    println!("{} there is no method for getting info from this chunk.", key);
                }
            }
        }
    }
    Ok(metadata)
}

pub fn empty_directory(dir: &str) -> Result<(), Box<dyn Error>> {
    if std::path::Path::new(dir).exists() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    } else {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}