use crate::whisper_core::services::{whisper_config_service, whisper_notifyme_service};

pub fn handle(message: &str, title: &str) {
    let current_path = std::env::current_dir();

    match current_path {
        Err(error) => {
            println!(
                "Something went wrong while getting the current path: {}",
                error.to_string()
            );
        }
        Ok(current_dir) => {
            let current_config = whisper_config_service::load_configuration_file(&current_dir);

            match current_config {
                Err(error) => {
                    println!("Something went wrong while getting your current whisper configuration file: {}", error.to_string());
                }
                Ok(current_settings) => {
                    let notifyme_settings = current_settings.notify_me_config;

                    match notifyme_settings {
                        None => {
                            println!("Your whisper configuration file does not contain a definition for NotifyMe");
                        }
                        Some(notifyme_settings) => {
                            whisper_notifyme_service::handle(
                                message.to_string(),
                                title.to_string(),
                                notifyme_settings.notify_me_token,
                            );
                        }
                    }
                }
            }
        }
    }
}
