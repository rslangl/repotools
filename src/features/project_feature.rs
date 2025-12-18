//! src/features/project_feature.rs

use std::{fmt, path::PathBuf};
use clap::Args;

use crate::app_config::AppConfig;

#[derive(Debug)]
pub enum ProjectFeatureError {
    Invalid(String),
    Write {
        path: PathBuf,
        source: std::io::Error
    }
    // Specific feature type errors
    
}

impl fmt::Display for ProjectFeatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectFeatureError::Invalid(e) => {
                write!(f, "{}", e)
            },
            _ => todo!()    // TODO: requires exhaustive match arms
        }
    }
}

#[derive(Args)]
pub struct ProjectFeatureArgs {

    #[arg(long = "type")]
    pub feature_type: String,
}

pub fn handle(args: ProjectFeatureArgs, config: AppConfig) -> Result<(), ProjectFeatureError> {

    Ok(())
}
