use std::path::PathBuf;

use crate::whisper_core::{
    exceptions::whisper_error::WhisperError, models::whisper_config::WhisperConfig,
    services::whisper_testfairy_service,
};

pub fn handle(
    current_settings: WhisperConfig,
    application_path: PathBuf,
) -> Result<(), WhisperError> {
    let testfairy_settings = current_settings.testfairy_config;

    match testfairy_settings {
        None => Err(WhisperError::ConfigMissingDefinition {
            module_definition: String::from(
                "Your whisper configuration file does not contain a definition for TestFairy",
            ),
        }),
        Some(tf_settings) => {
            whisper_testfairy_service::handle(
                tf_settings.testfairy_access_token,
                application_path,
            )?;
            Ok(())
        }
    }
}
