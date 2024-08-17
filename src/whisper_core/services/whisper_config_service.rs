use std::{fs, io::Write, path::PathBuf};

use crate::whisper_core::models::whisper_config::WhisperConfig;

pub fn handle_path(file_path: &mut PathBuf) {
    match file_path.try_exists() {
        Err(error) => {
            println!("Something went wrong {}", error.to_string())
        }
        Ok(result) => {
            if result {
                file_path.set_file_name("whisper");
                file_path.set_extension("toml");
                let exists = file_path.exists();

                if exists {
                    println!("Configuration file already exists, skipping creation...")
                } else {
                    println!("Configuration file doesn't exist, creating whisper.toml");
                    create_configuration_file(file_path);
                }
            } else {
                match file_path.to_str() {
                    Some(path) => {
                        println!("Path {} doesn't seem to exist", path)
                    }
                    None => println!("Malformed path or doesn't seem to exist"),
                }
            }
        }
    }
}

pub fn create_configuration_file(creation_path: &mut PathBuf) {
    let sample_config = WhisperConfig::new(
        String::from("whisper_sample"),
        String::from("example.com"),
        String::from("example.com"),
    );

    let serialized_value = toml::to_string(&sample_config).unwrap();

    let mut file_handler = fs::File::create_new(creation_path).unwrap();

    let result = file_handler.write_all(serialized_value.as_bytes());

    match result {
        Err(error) => println!(
            "Something went wrong while creating the file: {}",
            error.to_string()
        ),
        Ok(_) => {
            {
                match (&*&mut file_handler).flush() {
                    Ok(_) => println!("Configuration file created"),
                    Err(error) => println!(
                        "Something went wrong while flushing file handler: {}",
                        error.to_string()
                    ),
                }
            };
        }
    }
}
