//! src/features/resources/linter.rs

use std::{
    fmt,
    path::{Path, PathBuf},
};

use crate::{
    app_config::app_config::Linter,
    features::{ProjectFeatureError, project_feature::FeatureStrategy},
    utils::write,
};

#[derive(Debug)]
pub enum LinterResourceError {
    NotFound(String),
}

impl fmt::Display for LinterResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinterResourceError::NotFound(linter) => {
                write!(f, "Requested linter resource was not found: `{}`", linter)
            }
        }
    }
}

pub struct LinterResource {
    name: String,
    file: PathBuf,
}

impl LinterResource {
    pub fn new(name: String, linters: Vec<Linter>) -> Result<Self, LinterResourceError> {
        let linter = linters
            .iter()
            .find(|l| l.name == name.clone())
            .ok_or(LinterResourceError::NotFound(name.clone()))?;

        Ok(Self {
            name: name,
            file: linter.file_path.clone(),
        })
    }
}

impl FeatureStrategy for LinterResource {
    fn write_files(&self) -> Result<(), ProjectFeatureError> {
        write(&self.file.as_path(), None)?;
        Ok(())
    }
}
