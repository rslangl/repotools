//! src/main.rs

use clap::{Parser};
use crate::cli::{Cli, Command};

use repotools::cli;
use repotools::app_config::{app_config, ConfigError};
use repotools::initializers::{init_project, InitProjectError};
use repotools::features::{project_feature, ProjectFeatureError};

#[derive(Debug)]
enum AppError {
    Config(ConfigError),
    InitProject(InitProjectError),
    ProjectFeature(ProjectFeatureError)
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

impl From<ProjectFeatureError> for AppError {
    fn from(e: ProjectFeatureError) -> Self {
        AppError::ProjectFeature(e)
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
        },
        Command::ProjectFeature(args) => {
            if let Err(e) = project_feature::handle(args, config) {
                eprintln!("Could not add feature: {}", e)
            }
        }
    }

    Ok(())
}
