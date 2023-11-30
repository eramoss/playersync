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
    Play {
        player: Option<String>,
    },
    Pause {
        player: Option<String>,
    },
    Stop {
        player: Option<String>,
    },
    Next {
        player: Option<String>,
    },
    Previous {
        player: Option<String>,
    },
    Volume {
        level: i32,
        player: Option<String>,
    },
    Info,
    Seek {
        position: String,
        player: Option<String>,
    },
    List,
    Status {
        player: Option<String>,
    },
    Shuffle {
        player: Option<String>,
    },
    Position {
        player: Option<String>,
    },
}
