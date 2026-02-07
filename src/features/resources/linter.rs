//! src/features/feature_types/linter.rs

#[derive(Debug)]
pub enum LinterResourceError {
    NotFound,
}

impl fmt::Display for LinterResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinterResourceError::NotFound => {
                write!(f, "Requested linter resource was not found")
            }
        }
    }
}

pub struct LinterResource {
    name: String,
}

impl LinterResource {
    pub fn new(linter_name: String) -> Result<Self, LinterResourceError> {
        Ok(Self { name: linter_name })
    }
}

impl FeatureStrategy for LinterResource {
    fn write_templates(&self) -> Result<(), ProjectFeatureError> {
        create_files()?;
        Ok(())
    }
}
