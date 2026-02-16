//! src/features/resources/license.rs

use std::{fmt, path::PathBuf};

use crate::{
    app_config::app_config::License,
    features::{ProjectFeatureError, project_feature::FeatureStrategy},
    utils::write,
};

#[derive(Debug)]
pub enum LicenseResourceError {
    NotFound(String),
}

impl fmt::Display for LicenseResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LicenseResourceError::NotFound(license) => {
                write!(f, "Requested license resource was not found: `{}`", license)
            }
        }
    }
}

pub struct LicenseResource {
    name: String,
    file: PathBuf,
}

impl LicenseResource {
    pub fn new(name: String, licenses: Vec<License>) -> Result<Self, LicenseResourceError> {
        let license = licenses
            .iter()
            .find(|l| l.name == name.clone())
            .ok_or(LicenseResourceError::NotFound(name.clone()))?;

        Ok(Self {
            name: name,
            file: license.file_path.clone(),
        })
    }
}

impl FeatureStrategy for LicenseResource {
    fn write_files(self: Box<Self>) -> Result<(), ProjectFeatureError> {
        file_write::write(self.file, None)?;
        Ok(())
    }
}
