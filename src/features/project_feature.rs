//! src/features/project_feature.rs

use std::fmt;

use clap::Args;

use crate::{
    app_config::app_config::{AppConfig, Features},
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
            ProjectFeatureError::LicenseError(e) => {
                write!(f, "{}", e)
            }
            ProjectFeatureError::LinterError(e) => {
                write!(f, "{}", e)
            }
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

struct FeatureAddition {
    feature_strategy: Box<dyn FeatureStrategy>,
}

impl FeatureAddition {
    fn new(feature_strategy: Box<dyn FeatureStrategy>) -> Self {
        Self { feature_strategy }
    }

    fn add_feature(self) -> Result<(), ProjectFeatureError> {
        Ok(self.feature_strategy.write_files()?)
    }
}

pub trait FeatureStrategy {
    fn write_files(self: Box<Self>) -> Result<(), ProjectFeatureError>;
}

struct FeatureFactory;

impl FeatureFactory {
    fn new(
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
            _ => Err(ProjectFeatureError::Invalid("Unknown feature type".into())),
        }
    }
}

pub fn handle(args: ProjectFeatureArgs, config: AppConfig) -> Result<(), ProjectFeatureError> {
    let strategy: Box<dyn FeatureStrategy> =
        FeatureFactory::new(args.feature_function, args.feature_type, config.features)?;

    let addition: FeatureAddition = FeatureAddition::new(strategy);

    addition.add_feature()?;

    // let feature = match FeatureFactory::add_feature(
    //     args.feature_function,
    //     args.feature_type,
    //     config.features,
    // ) {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };

    Ok(())
}
