//! src/features/resources/license.rs

use std::fmt;

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
    pub fn new(name: String) -> Result<Self, LicenseResourceError> {
        // TODO: search through list of licenses
        Ok(Self { name: name })
    }
}
