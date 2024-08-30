use crate::whisper_core::models::whisper_config::WhisperConfig;
use crate::whisper_core::services::whisper_config_service::load_configuration_file;

pub fn handle(config_handler: impl Fn(WhisperConfig)) {
    let current_path = std::env::current_dir();

    match current_path {
        Err(error) => println!(
            "Something went wrong while getting the current path: {}",
            error.to_string()
        ),
        Ok(current_dir) => match load_configuration_file(&current_dir) {
            Err(error) => {
                println!("Something went wrong while getting your current whisper configuration file: {}", error.to_string());
            }
            Ok(configuration) => {
                config_handler(configuration);
            }
        },
    }
}
