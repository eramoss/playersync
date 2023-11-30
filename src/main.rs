use clap::Parser;
use playersync::cli;
fn main() {
    let args = cli::Args::parse();

    dbg!(args);
}
