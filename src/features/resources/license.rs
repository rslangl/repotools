//! src/features/resources/license.rs

use std::{fmt, path::Path};

use crate::{
    app_config::app_config::License,
    features::{ProjectFeatureError, project_feature::FeatureStrategy},
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
}

impl LicenseResource {
    pub fn new(name: String, licenses: Vec<License>) -> Result<Self, LicenseResourceError> {
        let licenses = licenses
            .iter()
            .find(|l| l.name == name.clone())
            .ok_or(LicenseResourceError::NotFound(name.clone()))?;

        Ok(Self { name: name })
    }
}

impl FeatureStrategy for LicenseResource {
    fn write_files(&self, source: &Path) -> Result<(), ProjectFeatureError> {
        todo!()
    }
}
