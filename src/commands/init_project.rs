//! commands/init_project.rs

use std::{collections::HashMap, fs::{self, File, ReadDir}, io::{Read, Write}};

use clap::{Args, Error};

use crate::app_config::app_config::ProjectTemplate;

const PROJECT_TYPE_MAVEN: &'static str = "MAVEN";
const PROJECT_TYPE_ANSIBLE: &'static str = "ANSIBLE";

#[derive(Args)]
pub struct InitProjectArgs {
    #[arg(long = "type")]
    pub project_type: String,
    #[arg(long)]
    pub profile: Option<String>
}

trait ProjectStrategy {
    fn write_templates(&self, read_dir: ReadDir) -> Result<(), String>;
}

struct MavenProject;

impl ProjectStrategy for MavenProject {
    fn write_templates(&self, read_dir: ReadDir) -> Result<(), String> {
        for entry in read_dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = entry.file_name();

                let mut source = File::open(path).map_err(|e| e.to_string())?;
                let mut content = String::new();
                source.read_to_string(&mut content).map_err(|e| e.to_string())?;

                let mut target_file = File::create(name).map_err(|e| e.to_string())?;
                target_file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

struct AnsibleProject;

impl ProjectStrategy for AnsibleProject {
    fn write_templates(&self, read_dir: ReadDir) -> Result<(), String> {
        Ok(())
    }
}

struct ProjectInitializer<T: ProjectStrategy> {
    initialize_strategy: T,
}

impl<T: ProjectStrategy> ProjectInitializer<T> {
    fn new(initialize_strategy: T) -> Self {
        Self { initialize_strategy }
    }

    fn initialize(&self, read_dir: ReadDir) -> Result<(), String> {
        self.initialize_strategy.write_templates(read_dir)
    }
}

struct ProjectFactory;

impl ProjectFactory {
    fn new(project_type: &str) -> Result<Box<dyn ProjectStrategy>, String> {
        match project_type {
            PROJECT_TYPE_MAVEN => Ok(Box::new(MavenProject)),
            PROJECT_TYPE_ANSIBLE => Ok(Box::new(AnsibleProject)),
            _ => Err("Unknown strategy".to_string()),
        }
    }
}

pub fn handle(args: InitProjectArgs, config: HashMap<String, toml::Value>) -> Result<(), String> {

    let serialized: &toml::Value = config.get("templates").ok_or("Could not find expected field 'templates'")?;

    let templates: Vec<ProjectTemplate> = serialized
        .clone()
        .try_into()
        .map_err(|e| format!("Could not deserialize list of templates from config file: {}", e.to_string()))?;

    let template = templates.iter().find(|p| {
        if let Some(profile) = &args.profile {
            p.name == args.project_type && p.profile == *profile
        } else {
            p.name == args.project_type && p.profile == "default"
        }
    }).ok_or("Could not find template".to_string())?;

    if let Ok(project) = ProjectFactory::new(args.project_type.to_uppercase().as_str()) {//;//.map_err(|e| e.to_string())?;
        if let Ok(read_dir) = fs::read_dir(template.template_files.as_path()) {
            project.write_templates(read_dir);
        };
    }

    Ok(())
}
