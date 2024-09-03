use crate::whisper_core::{
    models::whisper_config::WhisperConfig, services::whisper_notifyme_service,
};

pub fn handle(current_settings: WhisperConfig, message: &str, title: &str) {
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
