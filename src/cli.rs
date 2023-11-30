use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "eramoss",
    version = "0.1.0",
    about = "A command-line music player control utility.",
    long_about = "CLI app for controlling a music player. You can play, pause, stop, skip tracks, adjust volume, and more with ease."
)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Play { player: String },
    Pause { player: String },
    Stop { player: String },
    Next { player: String },
    Previous { player: String },
    Volume { level: i32, player: String },
    Info { player: String },
    Seek { position: String, player: String },
    List { player: String },
    Shuffle { player: String },
    Position { player: String },
}
