use clap::{arg, Command};

pub fn get_cmd() -> clap::Command {
    clap::Command::new("readme")
        .about("readme")
        .arg(clap::arg!(<PATH> "Path"))
}
