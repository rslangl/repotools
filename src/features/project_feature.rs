//! src/features/project_feature.rs

use std::{fmt, path::{Path, PathBuf}};
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

struct FeatureAddition<T: FeatureStrategy> {
    feature_strategy: T,
}

impl<T: FeatureStrategy> FeatureAddition<T> {
    fn new(feature_strategy: T) -> Self {
        Self { feature_strategy }
    }

    fn add(&self, source: &Path) -> Result<(), ProjectFeatureError> {
        self.feature_strategy.write_templates(source)
    }
}

pub trait FeatureStrategy {
    fn write_templates(&self, source: &Path) -> Result<(), ProjectFeatureError>;
}

struct FeatureFactory;

impl FeatureFactory {
    fn new(feature_type: &str) -> Result<(), ProjectFeatureError> {
        match feature_type {
            "LINTER" => Ok(()),
            _ => return Err(ProjectFeatureError::Invalid("Unknown feature type".into()))
        }
    }
}

#[derive(Args)]
pub struct ProjectFeatureArgs {

    #[arg(long = "type")]
    pub feature_type: String,
}

pub fn handle(args: ProjectFeatureArgs, config: AppConfig) -> Result<(), ProjectFeatureError> {

    let feature_type = "";

    match FeatureFactory::new(feature_type) {
        Ok(feature_template) => {
            // TODO: feature_template.write_templates()
        },
        Err(e) => return Err(e)
    }

    Ok(())
}
