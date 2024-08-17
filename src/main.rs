mod whisper_core;
mod whisper_ui;

use whisper_ui::prelude::*;

use clap::Parser;

fn main() {
    let args = whisper_ui::prelude::WhisperCommandApp::parse();
    dbg!(&args);

    match args.commands {
        WhisperCommandRepresentable::Init { path } => {
            whisper_ui::whisper_init_command::handle(path)
        }
    }
}
