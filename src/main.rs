use anyhow::{Context, Ok};
use clap::Parser;
use dbus::blocking::Connection;
use playersync::cli;
use std::{rc::Rc, time::Duration};

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let connection = Rc::new(Connection::new_session().context("create connection to dbus")?);

    match args.command {
        cli::Command::List => {
            let player_names = get_player_names(&connection)?;

            for name in player_names {
                println!("{}", name);
            }
        }
        cli::Command::PlayPause { player } => {
            let mut player_name = String::new();
            if let Some(player) = player {
                player_name = format!("org.mpris.MediaPlayer2.{}", player);
            } else {
                let player_names = get_player_names(&connection)?;

                player_name = player_names
                    .first()
                    .expect("Unable to get first player running")
                    .to_string();
            }
            let proxy = connection.with_proxy(
                player_name,
                "/org/mpris/MediaPlayer2",
                std::time::Duration::from_millis(5000),
            );
            proxy
                .method_call("org.mpris.MediaPlayer2.Player", "PlayPause", ())
                .context("Call playPause to dbus")?;
        }

        _ => {
            todo!()
        }
    }

    Ok(())
}

fn get_player_names(connection: &Connection) -> anyhow::Result<Vec<String>, anyhow::Error> {
    // Create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .context("send ListNames method to Dbus")?;

    let player_names = names
        .into_iter()
        .filter(|s| s.starts_with("org.mpris.MediaPlayer2"))
        .collect();

    Ok(player_names)
}
