#[derive(Debug)]
pub struct ConfigFileNotFoundError {}

impl ConfigFileNotFoundError {
    pub fn new() -> Self {
        Self {  }
    }
}

impl std::error::Error for ConfigFileNotFoundError {}

impl std::fmt::Display for ConfigFileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "A Whisper configuration file cannot be found, ensure a whisper configuration file is present in the current directory")
    }
}
