//! src/initializers/project_maven.rs

use std::{path::Path, collections::HashMap};

use crate::initializers::init_project::{Val, ProjectStrategy, create_files};

pub struct MavenProject {
    group_id: String,
    artifact_id: String
}

impl MavenProject {
    pub fn new(settings: HashMap<String, String>) -> Self {

        let group_id = settings
            .get("group_id")
            .cloned()
            .unwrap();
            // .ok_or("Expected Maven setting `groupId`")?;

        let artifact_id = settings
            .get("artifact_id")
            .cloned()
            .unwrap();
            // .ok_or("Expected Maven setting `artifactId`")?;

        Self {
            group_id: group_id,
            artifact_id: artifact_id
        }
    }

    fn get_properties(&self) -> HashMap<String, Val> {
        let mut properties = HashMap::new();
        properties.insert("group_id".to_string(), Val::Str(self.group_id.clone()));
        properties.insert("artifact_id".to_string(), Val::Str(self.artifact_id.clone()));
        properties
    }
}

impl ProjectStrategy for MavenProject {

    fn write_templates(&self, source: &Path) -> Result<(), String> {
        create_files(&source, &source, &MavenProject::get_properties(self));
        Ok(())
    }
}
