use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    name = "Whisper",
    version = "0.1.0",
    about = "Whisper is a simple CLI tool that automates common tasks in mobile software development",
    long_about = None
)]
pub struct WhisperCommandApp {
    #[command(subcommand)]
    pub commands: WhisperCommandRepresentable,
}

#[derive(Subcommand, Debug)]
pub enum WhisperCommandRepresentable {
    Init {
        #[arg(index = 1, help = "Init whisper configuration")]
        path: Option<PathBuf>,
    },
}

/*
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct WhisperCommandApp {
    #[command(subcommand)]
    init: WhisperInitCommand,
}

#[derive(Subcommand, Debug)]
pub enum WhisperInitCommand {
    Init(OwO),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct OwO {
    #[command(subcommand)]
    init: WhisperInitSubcommand,
}
*/
