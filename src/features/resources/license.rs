//! src/features/resources/license.rs

#[derive(Debug)]
pub enum LicenseResourceError {
    NotFound,
}

impl fmt::Display for LicenseResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>') -> fmt::Result {
        match self {
            LicenseResourceError::Notfound => {
                write!(f, "Requested license resource was not found")
            }
        }
    }
}

pub struct LicenseResourceError {
    name: String,
}

impl LicenseResourceError {
    pub fn new(license_name: String) -> Result<Self, LicenseResourceError> {
        Ok(Self{ name: license_name })
    }
}
