use anyhow::{Context, Ok};
use clap::Parser;
use dbus::blocking::Connection;
use playersync::cli;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let connection = Connection::new_session().context("create connection to dbus")?;

    // Create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = connection.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    match args.command {
        cli::Command::List => {
            let (names,): (Vec<String>,) = proxy
                .method_call("org.freedesktop.DBus", "ListNames", ())
                .context("send ListNames method to Dbus")?;
            names.iter().for_each(|s| {
                if s.starts_with("org.mpris.MediaPlayer2") {
                    println!("{}", s);
                }
            });
        }
        _ => {
            todo!()
        }
    }

    Ok(())
}
