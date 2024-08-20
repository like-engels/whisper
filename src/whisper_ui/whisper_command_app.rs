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
    pub commands: WhisperCommandMenu,
}

#[derive(Subcommand, Debug)]
pub enum WhisperCommandMenu {
    #[command(
        after_help = "Creates a whisper.toml configuration file where Whisper options can be set",
        about = "Create Whisper configuration file"
    )]
    Init {
        #[arg(index = 1, help = "Path to create configuration file")]
        path: Option<PathBuf>,
    },
    #[command(
        after_help = "Deliver notifications to Amazon Echo Devices using the NotifyMe Alexa skill",
        about = "Send notifications through NotifyMe to Amazon Echo devices"
    )]
    NotifyMe {
        #[arg(help = "Message content", long = "message", short = 'm')]
        message: String,
        #[arg(help = "Message title", long = "title", short = 't')]
        title: String,
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
