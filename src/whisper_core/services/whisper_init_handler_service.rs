use crate::whisper_core::exceptions::whisper_error::WhisperError;
use crate::whisper_core::models::whisper_config::WhisperConfig;
use crate::whisper_core::services::whisper_config_service::load_configuration_file;

pub fn handle() -> Result<WhisperConfig, WhisperError> {
    let current_path = std::env::current_dir()?;

    let configuration = load_configuration_file(&current_path)?;

    Ok(configuration)
}
