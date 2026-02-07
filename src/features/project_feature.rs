//! src/features/project_feature.rs

use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use clap::Args;

use crate::{
    app_config::app_config::AppConfig,
    features::resources::{
        LicenseResource, LicenseResourceError, LinterResource, LinterResourceError,
    },
};

#[derive(Debug)]
pub enum ProjectFeatureError {
    Invalid(String),
    // Specific feature type errors
    LicenseError(LicenseResourceError),
    LinterError(LinterResourceError),
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

    fn add(&self, source: &Path) -> Result<(), ProjectFeatureError> {
        self.feature_strategy.write_file(source)
    }
}

struct FeatureFactory;

impl FeatureFactory {
    fn new(
        feature_function: String,
        feature_type: String,
    ) -> Result<Box<dyn FeatureStrategy>, ProjectFeatureError> {
        match feature_function.as_str() {
            "LINTER" => Ok(Box::new(LinterResource::new(feature_type))?),
            "LICENSE" => Ok(Box::new(LicenseResource::new(feature_type))?),
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

pub fn create_files(source: &Path) -> Result<(), ProjectFeatureError> {
    let target = Path::new("."); // TODO:: using current dir for now

    let contents = fs::read_to_string(source)?;

    fs::write(target, contents).map_err(|e| ProjectFeatureError::Write {
        path: target.clone(),
        source: e,
    })?;

    Ok(())
}

pub fn handle(args: ProjectFeatureArgs, config: AppConfig) -> Result<(), ProjectFeatureError> {
    // Match input on available types of features; linter, license
    // let feature_function = config
    //     .features
    //     .iter()
    //     .find(|f| f == args.feature_function)
    //     .ok_or(ProjectFeatureError::Invalid(
    //         "Could not find feature".into(),
    //     ))?;
    //
    // Subsequent match on type; linter={YAML,Markdown}, license={MIT,GPL}
    // let feature_type = args.feature_type;

    match FeatureFactory::new(args.feature_function.to_uppercase(), args.feature_type) {
        Ok(feature) => {
            //feature.write_file()?;
            // TODO: whatever the fuck
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
