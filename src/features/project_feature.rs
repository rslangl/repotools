//! src/features/project_feature.rs

use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use clap::Args;

use crate::{
    app_config::app_config::{AppConfig, Features, Linter},
    features::resources::{
        LicenseResource, LicenseResourceError, LinterResource, LinterResourceError,
    },
    utils::file_writer::FileWriteError,
};

#[derive(Debug)]
pub enum ProjectFeatureError {
    Invalid(String),
    // Specific feature type errors
    LicenseError(LicenseResourceError),
    LinterError(LinterResourceError),
}

impl From<FileWriteError> for ProjectFeatureError {
    fn from(e: FileWriteError) -> Self {
        todo!()
    }
}

impl From<LicenseResourceError> for ProjectFeatureError {
    fn from(e: LicenseResourceError) -> Self {
        ProjectFeatureError::LicenseError(e)
    }
}

impl From<LinterResourceError> for ProjectFeatureError {
    fn from(e: LinterResourceError) -> Self {
        ProjectFeatureError::LinterError(e)
    }
}

impl fmt::Display for ProjectFeatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectFeatureError::Invalid(e) => {
                write!(f, "{}", e)
            }
            _ => todo!(), // TODO: requires exhaustive match arms
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

    fn add(&self) -> Result<(), ProjectFeatureError> {
        self.feature_strategy.write_files()
    }
}

pub trait FeatureStrategy {
    fn write_files(&self) -> Result<(), ProjectFeatureError>;
}

struct FeatureFactory;

impl FeatureFactory {
    fn add(
        feature_function: String,
        feature_type: String,
        features: Features,
    ) -> Result<Box<dyn FeatureStrategy>, ProjectFeatureError> {
        match feature_function.to_uppercase().as_str() {
            "LINTER" => Ok(Box::new(LinterResource::new(
                feature_type,
                features.linters,
            )?)),
            "LICENSE" => Ok(Box::new(LicenseResource::new(
                feature_type,
                features.licenses,
            )?)),
            _ => return Err(ProjectFeatureError::Invalid("Unknown feature type".into())),
        }
    }
}

#[derive(Args)]
pub struct ProjectFeatureArgs {
    #[arg(long = "function")]
    pub feature_function: String,

    #[arg(long = "type")]
    pub feature_type: String,
}

pub fn handle(args: ProjectFeatureArgs, config: AppConfig) -> Result<(), ProjectFeatureError> {
    match FeatureFactory::add(args.feature_function, args.feature_type, config.features) {
        Ok(feature) => {
            feature.write_files()?;
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
