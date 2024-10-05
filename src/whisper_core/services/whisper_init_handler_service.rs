use std::path::PathBuf;

use crate::whisper_core::exceptions::whisper_error::WhisperError;
use crate::whisper_core::models::whisper_config::WhisperConfig;
use crate::whisper_core::services::whisper_config_service::load_configuration_file;

pub fn handle() -> Result<WhisperConfig, WhisperError> {
    let mut current_path = std::env::current_dir()?;

    let mut config_file = PathBuf::new();
    config_file.set_file_name("whisper");
    config_file.set_extension("toml");

    current_path.push(config_file);

    let configuration = load_configuration_file(&current_path)?;

    Ok(configuration)
}
