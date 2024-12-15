use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Serialize)]
pub struct WhisperConfig {
    pub application_name: String,
    pub documentation_url: String,
    pub homepage_url: String,
    #[serde(alias = "environment")]
    pub environment_config: Option<WhisperEnvironmentConfig>,
    #[serde(alias = "accent")]
    pub accent_config: Option<WhisperAccentConfig>,
    #[serde(alias = "jenkins")]
    pub jenkins_config: Option<WhisperJenkinsConfig>,
    #[serde(alias = "doppler")]
    pub doppler_config: Option<WhisperDopplerConfig>,
    #[serde(alias = "notifyme")]
    pub notify_me_config: Option<WhisperNotifyMeConfig>,
    #[serde(alias = "testfairy")]
    pub testfairy_config: Option<WhisperTestFairyConfig>,
}

impl WhisperConfig {
    pub fn new(application_name: String, documentation_url: String, homepage_url: String) -> Self {
        Self {
            application_name,
            documentation_url,
            homepage_url,
            environment_config: None,
            accent_config: None,
            jenkins_config: None,
            doppler_config: None,
            notify_me_config: None,
            testfairy_config: None
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

#[derive(Deserialize, Serialize)]
pub struct WhisperTestFairyConfig {
    pub testfairy_access_token: String,
}
