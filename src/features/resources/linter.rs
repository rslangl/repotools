//! src/features/resources/linter.rs

use std::fmt;

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
}

impl LinterResource {
    pub fn new(name: String) -> Result<Self, LinterResourceError> {
        // TODO: do lookup to find desired linter
        Ok(Self { name: name })
    }
}
