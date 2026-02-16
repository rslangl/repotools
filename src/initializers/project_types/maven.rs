//! src/initializers/project_maven.rs

use std::{collections::HashMap, fmt, path::PathBuf};

use crate::{
    initializers::{
        init_project::{InitProjectError, ProjectStrategy},
        project_types::common::FileTemplate,
    },
    utils::file_writer,
};

#[derive(Debug)]
pub enum MavenProjectError {
    MissingProperty(String),
}

impl fmt::Display for MavenProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MavenProjectError::MissingProperty(property) => {
                write!(
                    f,
                    "Template creation error: missing property `{}`",
                    property
                )
            }
        }
    }
}

pub struct MavenProject {
    pub file_template: FileTemplate,
    pub settings: HashMap<String, file_writer::Val>,
    //     group_id: String,
    //     artifact_id: String,
}

impl MavenProject {
    pub fn new(
        template_files: PathBuf,
        settings: HashMap<String, String>,
    ) -> Result<Self, MavenProjectError> {
        let group_id = settings
            .get("group_id")
            .cloned()
            .ok_or(MavenProjectError::MissingProperty("group_id".into()))?;

        let artifact_id = settings
            .get("artifact_id")
            .cloned()
            .ok_or(MavenProjectError::MissingProperty("artifact_id".into()))?;

        let project_properties = HashMap::from([
            (
                String::from("artifact_id"),
                file_writer::Val::Str(artifact_id),
            ),
            (String::from("group_id"), file_writer::Val::Str(group_id)),
        ]);

        Ok(Self {
            file_template: FileTemplate::new(template_files),
            settings: project_properties,
        })
    }

    //     fn get_properties(&self) -> HashMap<String, file_writer::Val> {
    //         let mut properties = HashMap::new();
    //         properties.insert(
    //             "group_id".to_string(),
    //             file_writer::Val::Str(self.group_id.clone()),
    //         );
    //         properties.insert(
    //             "artifact_id".to_string(),
    //             file_writer::Val::Str(self.artifact_id.clone()),
    //         );
    //         properties
    //     }
}

impl ProjectStrategy for MavenProject {
    fn write_templates(self: Box<Self>) -> Result<(), InitProjectError> {
        file_writer::write(
            self.file_template.source_files,
            Some(self.settings), //Some(MavenProject::get_properties(self)),
        )?;
        Ok(())
    }
}
