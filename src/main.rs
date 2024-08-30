mod whisper_core;
mod whisper_ui;

use whisper_ui::prelude::*;

use clap::Parser;

fn main() {
    let args = whisper_ui::prelude::WhisperCommandApp::parse();

    match args.commands {
        WhisperCommandMenu::Init { path } => whisper_ui::whisper_init_command::handle(path),
        WhisperCommandMenu::NotifyMe { message, title } => {
            whisper_ui::whisper_init_handler::handle(|configurations| {
                whisper_ui::whisper_notifyme_command::handle(configurations, &message, &title)
            })
        }
    }
}
