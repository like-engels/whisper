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
    #[command(
        after_help = "Uploads an APK, IPA or ZIP file to TestFairy",
        about = "Upload apps to TestFairy"
    )]
    #[command(name = "testfairy")]
    TestFairy {
        #[arg(help = "Path to application binary", long = "app-file", short = 'f')]
        application_path: PathBuf,
    },
}
