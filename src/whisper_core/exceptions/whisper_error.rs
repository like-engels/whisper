use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhisperError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    DE(#[from] toml::de::Error),
    #[error(transparent)]
    SER(#[from] toml::ser::Error),
    #[error(transparent)]
    REQ(#[from] reqwest::Error),
    #[error(transparent)]
    EC(#[from] color_eyre::Report),
    #[error("A Whisper configuration file cannot be found, ensure a whisper configuration file is present in the current directory")]
    ConfigFileNotFound,
    #[error("A definition for {module_definition:?} cannot be found in your configuration file")]
    ConfigMissingDefinition { module_definition: String },
    #[error("File {file_path:?} not found")]
    FileNotFound { file_path: String },
    #[error("File {file_path:?} has incorrect extension: Expected {expected_extension:?}")]
    IncorrectFileExtension {
        file_path: String,
        expected_extension: String,
    },
    #[error("Something went wrong while calling API {endpoint_url:?} from {service_name:?}")]
    NetworkRequestError {
        endpoint_url: String,
        service_name: String,
    },
}
