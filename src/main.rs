//! src/main.rs

use clap::{Parser};
use crate::cli::{Cli, Command};

use repotools::cli;
use repotools::app_config::{app_config, ConfigError};
use repotools::initializers::{init_project, InitProjectError};

#[derive(Debug)]
enum AppError {
    Config(ConfigError),
    InitProject(InitProjectError)
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

impl From<InitProjectError> for AppError {
    fn from(e: InitProjectError) -> Self {
        AppError::InitProject(e)
    }
}

fn main() -> Result<(), AppError> {

    let cli = Cli::parse();

    let config = app_config::get_config(cli.global.config_path)?;

    match cli.command {
        Command::InitProject(args) => {
            if let Err(e) = init_project::handle(args, config) {
                eprintln!("Could not initialize project: {}", e)
            }
        }
        // TODO: Command::AddFeature(args)
    }

    Ok(())
}
