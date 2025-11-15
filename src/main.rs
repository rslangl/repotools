mod cli;
mod commands;
mod app_config;

use clap::Parser;
use crate::cli::{Cli, Command};

fn main() {

    let cli = Cli::parse();

    let config = app_config::get_config(std::path::PathBuf::from(cli.global.config_path));

    match cli.command {
        Command::InitProject(args) => commands::init_project::handle(args) // for global args, add &cli.global
    }
}
