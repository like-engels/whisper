use crate::{
    whisper_core::exceptions::whisper_error::WhisperError,
    whisper_ui::indicators::progress_bar::{create_progress_bar, create_progress_reader},
};
use reqwest::blocking::{multipart, Client};
use std::{fs::File, path::PathBuf, time::Duration};

const TESTFAIRY_API_ENDPOINT: &str = "https://upload.testfairy.com/api/upload";
const MAX_RETRIES: u8 = 3;
const INITIAL_TIMEOUT: u64 = 120;

pub fn handle(access_token: String, app_binary: PathBuf) -> Result<(), WhisperError> {
    let file_path = app_binary.display().to_string();
    validate_file(&app_binary)?;

    let file_size = app_binary.metadata()?.len();
    let file = File::open(&app_binary)?;

    let file_name = app_binary
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| WhisperError::FileNotFound {
            file_path: file_path.clone(),
        })?
        .to_owned();

    println!("Uploading file '{}' to TestFairy...", file_path);

    let http_client = Client::builder()
        .timeout(Duration::from_secs(INITIAL_TIMEOUT))
        .build()?;

    upload_with_retries(&http_client, access_token, file, file_name, file_size)
}

fn validate_file(app_binary: &PathBuf) -> Result<(), WhisperError> {
    let file_path = app_binary.display().to_string();

    if !app_binary.is_file() {
        return Err(WhisperError::FileNotFound { file_path });
    }

    let valid_extensions = ["apk", "zip", "ipa"];
    let extension = app_binary
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();

    if !valid_extensions.contains(&extension.as_str()) {
        return Err(WhisperError::IncorrectFileExtension {
            file_path,
            expected_extension: valid_extensions.join(", "),
        });
    }

    Ok(())
}

fn upload_with_retries(
    http_client: &Client,
    access_token: String,
    file: File,
    file_name: String,
    file_size: u64,
) -> Result<(), WhisperError> {
    let mut retries = 0;
    let mut timeout = Duration::from_secs(INITIAL_TIMEOUT);

    while retries <= MAX_RETRIES {
        let progress_bar = create_progress_bar(file_size);
        let progress_reader = create_progress_reader(file.try_clone()?, progress_bar)?;

        let file_part = multipart::Part::reader(progress_reader).file_name(file_name.clone());
        let multipart_request = multipart::Form::new()
            .text("api_key", access_token.clone())
            .part("file", file_part);

        match http_client
            .post(TESTFAIRY_API_ENDPOINT)
            .multipart(multipart_request)
            .send()
        {
            Ok(response) if response.status().is_success() => {
                println!("File successfully uploaded to TestFairy.");
                return Ok(());
            }
            Ok(response) if response.status() == 105 => {
                return Err(WhisperError::NetworkRequestError {
                    endpoint_url: TESTFAIRY_API_ENDPOINT.to_string(),
                    service_name:
                        "TestFairy Build Upload failure [Reason: Invalid file has been uploaded]"
                            .to_string(),
                });
            }
            Ok(_) => {
                return Err(WhisperError::NetworkRequestError {
                    endpoint_url: TESTFAIRY_API_ENDPOINT.to_string(),
                    service_name: "TestFairy Build Upload".to_string(),
                });
            }
            Err(err) if err.is_timeout() && retries < MAX_RETRIES => {
                retries += 1;
                timeout *= 2;
                println!(
                    "Upload attempt {} timed out. Retrying in {} seconds...",
                    retries,
                    timeout.as_secs()
                );
                continue;
            }
            Err(err) => Err(err)?,
        }
    }

    Err(WhisperError::NetworkRequestError {
        endpoint_url: TESTFAIRY_API_ENDPOINT.to_string(),
        service_name: "TestFairy Build Upload [Max retries exceeded]".to_string(),
    })
}
