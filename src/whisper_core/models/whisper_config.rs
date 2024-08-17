use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Serialize)]
pub struct WhisperConfig {
    pub application_name: String,
    pub documentation_url: String,
    pub homepage_url: String,
    pub environment: Option<WhisperEnvironmentConfig>,
    pub accent_config: Option<WhisperAccentConfig>,
    pub jenkins_config: Option<WhisperJenkinsConfig>,
    pub doppler_config: Option<WhisperDopplerConfig>,
    pub notify_me_config: Option<WhisperNotifyMeConfig>,
}

impl WhisperConfig {
    pub fn new(application_name: String, documentation_url: String, homepage_url: String) -> Self {
        Self {
            application_name,
            documentation_url,
            homepage_url,
            environment: None,
            accent_config: None,
            jenkins_config: None,
            doppler_config: None,
            notify_me_config: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct WhisperEnvironmentConfig {
    pub environment_name: String,
    pub environment_description: String,
}

#[derive(Deserialize, Serialize)]
pub struct WhisperAccentConfig {
    pub accent_endpoint: String,
    pub supported_languages: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct WhisperJenkinsConfig {
    pub pipeline_definition_file: String,
}

#[derive(Deserialize, Serialize)]
pub struct WhisperDopplerConfig {
    pub doppler_token: String,
    pub doppler_command_representable: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct WhisperNotifyMeConfig {
    pub notify_me_token: String,
}
