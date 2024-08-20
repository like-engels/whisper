use std::{fs, io::Write, path::PathBuf};

use crate::whisper_core::exceptions::whisper_error::WhisperError;
use crate::whisper_core::models::whisper_config::WhisperConfig;

pub fn handle_path(file_path: &PathBuf) {
    match file_path.try_exists() {
        Err(error) => {
            println!("Something went wrong {}", error.to_string())
        }
        Ok(result) => {
            if result {
                let mut configuration_file_path = file_path.clone();
                configuration_file_path.set_file_name("whisper");
                configuration_file_path.set_extension("toml");
                let exists = configuration_file_path.exists();

                if exists {
                    println!("Configuration file already exists, skipping creation...")
                } else {
                    println!("Configuration file doesn't exist, creating whisper.toml");
                    match create_configuration_file(configuration_file_path) {
                        Ok(_) => {
                            println!("Configuration file created successfuly");
                        }
                        Err(error) => {
                            println!(
                                "Something went wrong while creating the configuration file: {}",
                                error.to_string()
                            )
                        }
                    }
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

pub fn load_configuration_file(path: &PathBuf) -> Result<WhisperConfig, WhisperError> {
    let mut config_file_path = path.clone();

    let mut config_file = PathBuf::new();
    config_file.set_file_name("whisper");
    config_file.set_extension("toml");

    config_file_path.push(config_file);

    let exists = config_file_path.try_exists()?;

    if exists {
        let file_content = fs::read_to_string(config_file_path)?;

        let configuration: WhisperConfig = toml::from_str(&file_content)?;

        return Ok(configuration);
    } else {
        return Err(WhisperError::ConfigFileNotFound);
    }
}

fn create_configuration_file(creation_path: PathBuf) -> Result<(), WhisperError> {
    let sample_config = WhisperConfig::new(
        String::from("whisper_sample"),
        String::from("example.com"),
        String::from("example.com"),
    );

    let serialized_value = toml::to_string(&sample_config)?;

    let mut file_handler = fs::File::create_new(creation_path)?;

    file_handler.write_all(serialized_value.as_bytes())?;

    file_handler.flush()?;

    Ok(())
}
