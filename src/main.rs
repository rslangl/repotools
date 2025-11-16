mod cli;
mod commands;
mod app_config;

use clap::Parser;
use crate::cli::{Cli, Command};

fn main() {

    let cli = Cli::parse();

    let config = app_config::get_config(cli.global.config_path).expect("Could not load app config");

    match cli.command {
        Command::InitProject(args) => commands::init_project::handle(args, config) // for global args, add &cli.global
    }
}
