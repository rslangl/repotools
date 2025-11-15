mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Command::InitProject(args) => commands::init_project::handle(args) // for global args, add &cli.global
    }
}
