//! src/initializers/mod.rs

mod project_ansible;
mod project_maven;
pub mod init_project;

//use init_project::{Val, create_files};
pub use init_project::{InitProjectArgs, handle};

