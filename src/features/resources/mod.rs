//! src/features/resources/mod.rs

pub mod license;
pub mod linter;

pub use license::{LicenseResource, LicenseResourceError};
pub use linter::{LinterResource, LinterResourceError};
