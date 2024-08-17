use std::path::PathBuf;

use super::super::whisper_core::services::*;

pub fn handle(file_path: Option<PathBuf>) {
    match file_path {
        None => println!("No path was indicated"),
        Some(file_path) => {
            let mut path = file_path;
            whisper_config_service::handle_path(&mut path);
        }
    }
}
