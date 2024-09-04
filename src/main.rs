mod whisper_core;
mod whisper_ui;

use color_eyre::Result;
use whisper_core::exceptions::whisper_error::WhisperError;
use whisper_ui::prelude::*;

use clap::Parser;

fn main() -> Result<(), WhisperError> {
    let args = whisper_ui::prelude::WhisperCommandApp::parse();

    color_eyre::install()?;

    if let WhisperCommandMenu::Init { path } = args.commands {
        whisper_ui::whisper_init_command::handle(path);
        return Ok(());
    }
    
    let configurations = if let Some(custom_config_path) = &args.config {
        whisper_core::services::whisper_config_service::load_configuration_file(custom_config_path)?
    } else {
        whisper_core::services::whisper_init_handler_service::handle()?
    };

    match args.commands {
        WhisperCommandMenu::Init { .. } => {
            println!("You've found the lost case")
        }
        WhisperCommandMenu::NotifyMe { message, title } => {
            whisper_ui::whisper_notifyme_command::handle(configurations, &message, &title)
        }
    }

    Ok(())
}
