//! src/main.rs

use std::fmt;

use clap::Parser;

use crate::cli::{Cli, Command};

use repotools::{
    app_config::{CacheError, ConfigError, app_cache, app_config},
    cli,
    features::{ProjectFeatureError, project_feature},
    initializers::{InitProjectError, init_project},
};

#[derive(Debug)]
enum AppError {
    Config(ConfigError),
    Cache(CacheError),
    InitProject(InitProjectError),
    ProjectFeature(ProjectFeatureError),
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

impl From<CacheError> for AppError {
    fn from(e: CacheError) -> Self {
        AppError::Cache(e)
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

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => {
                write!(f, "ConfigErr: {}", e)
            }
            AppError::InitProject(e) => {
                write!(f, "InitErr: {}", e)
            }
            AppError::ProjectFeature(e) => {
                write!(f, "FeatureErr: {}", e)
            }
            AppError::Cache(e) => {
                write!(f, "CacheErr: {}", e)
            }
        }
    }
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    let config = app_config::get_config(cli.global.config_path)?;
    let cache = app_cache::get_cache(cli.global.cache_path)?;

    match cli.command {
        Command::InitProject(args) => {
            if let Err(e) = init_project::handle(args, config, cache) {
                eprintln!("Could not initialize project: {}", e)
            }
        }
        Command::ProjectFeature(args) => {
            if let Err(e) = project_feature::handle(args, config) {
                eprintln!("Could not add feature: {}", e)
            }
        }
    }

    Ok(())
}
