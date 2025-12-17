//! src/initializers/project_types/mod.rs

pub mod ansible;
pub mod maven;

pub use ansible::AnsibleProject;
pub use maven::{MavenProject, MavenProjectError};
