//! src/initializers/project_types/flux.rs

use std::{collections::HashMap, fmt, path::PathBuf};

use crate::{
    initializers::{
        init_project::{InitProjectError, ProjectStrategy},
        project_types::common::FileTemplate,
    },
    utils::file_writer,
};

#[derive(Debug)]
pub enum FluxProjectError {
    MissingProperty(String),
}

pub struct FluxProject {
    pub file_template: FileTemplate,
    pub settings: HashMap<String, file_writer::Val>,
}

impl FluxProject {
    pub fn new(
        template_files: PathBuf,
        settings: HashMap<String, String>,
    ) -> Result<Self, FluxProjectError> {
        let project_properties: HashMap<String, file_write::Val> = HashMap::from([
            (
                String::from("repo_name"),
                file_writer::Val::Seq(
                    settings
                        .get("repo_name")
                        .clone()
                        .ok_or_else(|| FluxProjectError::MissingProperty("repo_name".into()))?
                        .map(|s| file_writer::Val::Str(s.trim().to_string()))
                )
        )
        ])
    }
}

