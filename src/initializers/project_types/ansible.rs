//! src/initializers/project_ansible.rs

use std::{collections::HashMap, fmt, path::Path};

use crate::{
    initializers::init_project::{InitProjectError, ProjectStrategy},
    utils::file_writer,
};

#[derive(Debug)]
pub enum AnsibleProjectError {
    MissingProperty(String),
}

impl fmt::Display for AnsibleProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnsibleProjectError::MissingProperty(property) => {
                write!(
                    f,
                    "Template creation error: missing property `{}`",
                    property
                )
            }
        }
    }
}

pub struct AnsibleProject {
    // Host tuples are FQDN/hostnames and IP
    hosts: Vec<String>,
    roles: Vec<String>,
}

impl AnsibleProject {
    pub fn new(settings: HashMap<String, String>) -> Result<Self, AnsibleProjectError> {
        let hosts = settings
            .get("hosts")
            .cloned()
            .ok_or(AnsibleProjectError::MissingProperty("hosts".into()))?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let roles = settings
            .get("roles")
            .cloned()
            .ok_or(AnsibleProjectError::MissingProperty("roles".into()))?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Self {
            hosts: hosts,
            roles: roles,
        })
    }

    fn get_properties(&self) -> HashMap<String, file_writer::Val> {
        let mut properties = HashMap::new();
        properties.insert(
            "roles".to_string(),
            file_writer::Val::Seq(
                self.roles
                    .clone()
                    .into_iter()
                    .map(file_writer::Val::Str)
                    .collect(),
            ),
        );
        properties.insert(
            "hosts".to_string(),
            file_writer::Val::Seq(
                self.hosts
                    .clone()
                    .into_iter()
                    .map(file_writer::Val::Str)
                    .collect(),
            ),
        );
        properties
    }
}

impl ProjectStrategy for AnsibleProject {
    fn write_templates(&self, source: &Path) -> Result<(), InitProjectError> {
        file_writer::write(&source, Some(AnsibleProject::get_properties(self)))?;
        Ok(())
    }
}
