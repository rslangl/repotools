//! commands/init_project.rs

use std::{collections::HashMap, fs::{self, File, ReadDir}, io::{Read, Write}, str::FromStr};

use clap::{Args, Error};

use crate::app_config::app_config::{AppConfig, ProjectTemplate};

const PROJECT_TYPE_MAVEN: &'static str = "MAVEN";
const PROJECT_TYPE_ANSIBLE: &'static str = "ANSIBLE";

#[derive(Clone)]
pub struct ProjectSetting {
    pub key: String,
    pub val: String
}

impl FromStr for ProjectSetting {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k,v) = s.split_once('=').ok_or("Expected KEY=VALUE")?;
        Ok(Self {
            key: k.into(),
            val: v.into()
        })
    }
}

#[derive(Args)]
pub struct InitProjectArgs {

    #[arg(long = "type")]
    pub project_type: String,

    #[arg(long)]
    pub profile: Option<String>,

    #[arg(long)]
    pub settings: Option<Vec<ProjectSetting>>
}

trait ProjectStrategy {
    //fn new(&self, settings: HashMap<String, String>) -> Result<Self, String>;
    fn write_templates(&self, read_dir: ReadDir) -> Result<(), String>;
}

struct MavenProject {
    group_id: String,
    artifact_id: String

}

impl MavenProject {
    fn new(settings: HashMap<String, String>) -> Self {

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
}

impl ProjectStrategy for MavenProject {

    fn write_templates(&self, read_dir: ReadDir) -> Result<(), String> {
        for entry in read_dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = entry.file_name();

                let mut source = File::open(path).map_err(|e| e.to_string())?;
                let mut content = String::new();
                source.read_to_string(&mut content).map_err(|e| e.to_string())?;

                // TODO: use Tera to render template with group_id and artifact_id

                let mut target_file = File::create(name).map_err(|e| e.to_string())?;
                target_file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

// struct AnsibleProject;
//
// // impl ProjectStrategy for AnsibleProject {
//     fn write_templates(&self, read_dir: ReadDir) -> Result<(), String> {
//         Ok(())
//     }
// }

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
    fn new(project_type: &str, settings: HashMap<String, String>) -> Result<Box<dyn ProjectStrategy>, String> {
        match project_type {
            PROJECT_TYPE_MAVEN => Ok(Box::new(MavenProject::new(settings))),
            // PROJECT_TYPE_ANSIBLE => Ok(Box::new(AnsibleProject)),
            _ => Err("Unknown strategy".to_string()),
        }
    }
}

pub fn handle(args: InitProjectArgs, config: AppConfig) -> Result<(), String> {

    let template = config.templates.iter().find(|p| {
        if let Some(profile) = &args.profile {
            p.name == args.project_type && p.profile == *profile
        } else {
            p.name == args.project_type && p.profile == "default"
        }
    }).ok_or("Could not find template".to_string())?;

    // Convert custom key=value settings into map
    // for easier lookup
    let settings = match args.settings {
        Some(s) => {
            s.into_iter()
                .map(|kv| (kv.key, kv.val))
                .collect()
        },
        None => HashMap::new()
    };

    if let Ok(project) = ProjectFactory::new(args.project_type.to_uppercase().as_str(), settings) {
        if let Ok(read_dir) = fs::read_dir(template.template_files.as_path()) {
            project.write_templates(read_dir);
        };
    }

    Ok(())
}
