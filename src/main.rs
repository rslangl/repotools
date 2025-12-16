//! src/main.rs

use clap::{Parser};
use crate::cli::{Cli, Command};

use repotools::cli;
use repotools::app_config::app_config;
use repotools::initializers::init_project;

fn main() {

    let cli = Cli::parse();

    let config = app_config::get_config(cli.global.config_path).expect("Could not load app config");

    match cli.command {
        Command::InitProject(args) => init_project::handle(args, config).expect("Could not initialize project") // for global args, add &cli.global
    }
}
