use std::path::PathBuf;

use crate::whisper_core::services::*;

pub fn handle(file_path: Option<PathBuf>) {
    match file_path {
        None => println!("No path was indicated"),
        Some(file_path) => {
            whisper_config_service::handle_path(&file_path);
        }
    }
}
