use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhisperError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    DE(#[from] toml::de::Error),
    #[error(transparent)]
    SER(#[from] toml::ser::Error),
    #[error("A Whisper configuration file cannot be found, ensure a whisper configuration file is present in the current directory")]
    ConfigFileNotFound,
}
