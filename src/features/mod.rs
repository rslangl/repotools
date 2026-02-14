//! src/features/mod.rs

mod resources;

pub mod project_feature;

pub use project_feature::{ProjectFeatureArgs, ProjectFeatureError, handle};
