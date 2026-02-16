//! src/initializers/project_types/common.rs

use std::path::PathBuf;

// Using composition in the form of a shared struct in order to avoid repeating myself for all
// project types
pub struct FileTemplate {
    pub source_files: PathBuf,
}

impl FileTemplate {
    pub fn new(template_files: PathBuf) -> Self {
        Self {
            source_files: template_files,
        }
    }
}
