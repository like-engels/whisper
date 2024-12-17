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

    let http_client = Client::builder().build()?;

    upload_with_retries(
        &http_client,
        TESTFAIRY_API_ENDPOINT,
        Duration::from_secs(INITIAL_TIMEOUT),
        access_token,
        file,
        file_name,
        file_size,
    )
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
    endpoint_url: &str,
    timeout: Duration,
    access_token: String,
    file: File,
    file_name: String,
    file_size: u64,
) -> Result<(), WhisperError> {
    let mut retries = 0;
    let mut timeout = timeout;

    while retries < MAX_RETRIES {
        let progress_bar = create_progress_bar(file_size);
        let progress_reader = create_progress_reader(file.try_clone()?, progress_bar)?;

        let file_part = multipart::Part::reader(progress_reader).file_name(file_name.clone());
        let multipart_request = multipart::Form::new()
            .text("api_key", access_token.clone())
            .part("file", file_part);

        match http_client
            .post(endpoint_url)
            .timeout(timeout)
            .multipart(multipart_request)
            .send()
        {
            Ok(response) if response.status().is_success() => {
                println!("File successfully uploaded to TestFairy.");
                return Ok(());
            }
            Ok(response) if response.status() == 105 => {
                return Err(WhisperError::NetworkRequestError {
                    endpoint_url: endpoint_url.to_string(),
                    service_name:
                        "TestFairy Build Upload failure [Reason: Invalid file has been uploaded]"
                            .to_string(),
                });
            }
            Ok(_) => {
                return Err(WhisperError::NetworkRequestError {
                    endpoint_url: endpoint_url.to_string(),
                    service_name: "TestFairy Build Upload".to_string(),
                });
            }
            Err(err) if err.is_timeout() && retries < MAX_RETRIES => {
                retries += 1;
                timeout = std::cmp::min(timeout * 2, Duration::from_secs(240));

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
        endpoint_url: endpoint_url.to_string(),
        service_name: "TestFairy Build Upload [Max retries exceeded]".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::whisper_core::exceptions::whisper_error::WhisperError;
    use reqwest::blocking::Client;
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::Duration;
    use std::{fs::File, thread};
    use tempfile::{tempdir, tempfile};

    #[test]
    fn test_validate_file_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.apk");
        File::create(&file_path)
            .unwrap()
            .write_all(b"test")
            .unwrap();

        let result = super::validate_file(&file_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_file_file_not_found() {
        let non_existent_path = PathBuf::from("nonexistent.file");
        let result = super::validate_file(&non_existent_path);
        assert!(matches!(result, Err(WhisperError::FileNotFound { .. })));
    }

    #[test]
    fn test_validate_file_incorrect_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path)
            .unwrap()
            .write_all(b"test")
            .unwrap();

        let result = super::validate_file(&file_path);
        assert!(matches!(
            result,
            Err(WhisperError::IncorrectFileExtension { .. })
        ));
    }

    #[test]
    fn test_handle_file_not_found() {
        let non_existent_path = PathBuf::from("nonexistent.file");
        let result = super::handle("token".to_string(), non_existent_path);
        assert!(matches!(result, Err(WhisperError::FileNotFound { .. })));
    }

    #[test]
    fn test_upload_with_retries_success() {
        let mut server = mockito::Server::new();
        let mock = server.mock("POST", "/api/upload").with_status(201).create();

        let client = Client::builder().build().unwrap();
        let mut tmp_file = tempfile().unwrap();
        writeln!(tmp_file, "test content").unwrap();
        let file_size = tmp_file.metadata().unwrap().len();
        let file_name = "test_file".to_string();

        let result = super::upload_with_retries(
            &client,
            &format!("http://{}/api/upload", server.host_with_port()),
            Duration::from_secs(1),
            "token".to_string(),
            tmp_file,
            file_name,
            file_size,
        );

        assert!(result.is_ok());
        mock.assert();
    }

    #[test]
    fn test_upload_with_retries_timeout() {
        let mut server = mockito::Server::new();

        // Create a mock
        let mock = server.mock("POST", "/api/upload").with_status(408).create();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(120)); // Simulate 3-second delay
            mock.assert();
        });

        let client = Client::builder().build().unwrap();
        let file = File::open("Cargo.toml").unwrap(); // Assuming there's a file named Cargo.toml for testing
        let file_size = file.metadata().unwrap().len();
        let file_name = "Cargo.toml".to_string();

        let result = super::upload_with_retries(
            &client,
            &format!("http://{}/api/upload", server.host_with_port()),
            Duration::from_secs(20),
            "token".to_string(),
            file,
            file_name,
            file_size,
        );
        assert!(matches!(
            result,
            Err(WhisperError::NetworkRequestError { .. })
        ));
    }

    #[test]
    fn test_upload_with_retries_invalid_file() {
        let mut server = mockito::Server::new();

        // Create a mock
        let mock = server.mock("POST", "/api/upload").with_status(105).create();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(120)); // Simulate 3-second delay
            mock.assert();
        });

        let client = Client::builder().build().unwrap();

        let file = File::open("Cargo.toml").unwrap(); // Assuming there's a file named Cargo.toml for testing
        let file_size = file.metadata().unwrap().len();
        let file_name = "Cargo.toml".to_string();

        let result = super::upload_with_retries(
            &client,
            &format!("http://{}/api/upload", server.host_with_port()),
            Duration::from_secs(30),
            "token".to_string(),
            file,
            file_name,
            file_size,
        );
        assert!(matches!(
            result,
            Err(WhisperError::NetworkRequestError { .. })
        ));
    }
}
