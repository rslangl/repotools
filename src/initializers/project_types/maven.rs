//! src/initializers/project_maven.rs

use std::{fmt, path::Path, collections::HashMap};

use crate::initializers::init_project::{Val, ProjectStrategy, InitProjectError, create_files};

#[derive(Debug)]
pub enum MavenProjectError {
    MissingProperty(String)
}

impl fmt::Display for MavenProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MavenProjectError::MissingProperty(property) => {
                write!(f, "Template creation error: missing property `{}`", property)
            }
        }
    }
}

pub struct MavenProject {
    group_id: String,
    artifact_id: String
}

impl MavenProject {
    pub fn new(settings: HashMap<String, String>) -> Result<Self, MavenProjectError> {

        let group_id = settings
            .get("group_id")
            .cloned()
            .ok_or(MavenProjectError::MissingProperty("group_id".into()))?;
            //.unwrap();
            // .ok_or("Expected Maven setting `groupId`")?;

        let artifact_id = settings
            .get("artifact_id")
            .cloned()
            .ok_or(MavenProjectError::MissingProperty("artifact_id".into()))?;
            //.unwrap();
            // .ok_or("Expected Maven setting `artifactId`")?;

        Ok(Self {
            group_id: group_id,
            artifact_id: artifact_id
        })
    }

    fn get_properties(&self) -> HashMap<String, Val> {
        let mut properties = HashMap::new();
        properties.insert("group_id".to_string(), Val::Str(self.group_id.clone()));
        properties.insert("artifact_id".to_string(), Val::Str(self.artifact_id.clone()));
        properties
    }
}

impl ProjectStrategy for MavenProject {

    fn write_templates(&self, source: &Path) -> Result<(), InitProjectError> {
        create_files(&source, &source, &MavenProject::get_properties(self))?;
        Ok(())
    }
}
