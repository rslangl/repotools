//! src/initializers/project_ansible.rs

use std::{path::Path, collections::HashMap};

use crate::initializers::init_project::{Val, ProjectStrategy, create_files};

pub struct AnsibleProject {
    // Host tuples are FQDN/hostnames and IP
    hosts: Vec<String>,
    roles: Vec<String>
}

impl AnsibleProject {
    pub fn new(settings: HashMap<String, String>) -> Self {

        let hosts = settings
            .get("hosts")
            .cloned()
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let roles = settings
            .get("roles")
            .cloned()
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Self{
            hosts: hosts,
            roles: roles
        }
    }

    fn get_properties(&self) -> HashMap<String, Val> {
        let mut properties = HashMap::new();
        properties.insert("roles".to_string(), Val::Seq(self.roles.clone().into_iter().map(Val::Str).collect()));
        properties.insert("hosts".to_string(), Val::Seq(self.hosts.clone().into_iter().map(Val::Str).collect()));
        properties
    }
}

impl ProjectStrategy for AnsibleProject {
    fn write_templates(&self, source: &Path) -> Result<(), String> {
        create_files(&source, &source, &AnsibleProject::get_properties(self));
        Ok(())
    }
}
