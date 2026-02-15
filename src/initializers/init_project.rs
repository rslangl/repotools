//! src/initializers/init_project.rs

use std::{collections::HashMap, fmt, path::Path, str::FromStr};

use clap::Args;

use crate::app_config::app_config::AppConfig;
use crate::initializers::project_types::{
    ansible::{AnsibleProject, AnsibleProjectError},
    maven::{MavenProject, MavenProjectError},
};
use crate::utils::file_writer::FileWriteError;

#[derive(Debug)]
pub enum InitProjectError {
    Invalid(String),
    // Specific project type errors
    MavenProject(MavenProjectError),
    AnsibleProject(AnsibleProjectError),
}

// TODO: the fuck am I supposed to do with this
impl From<FileWriteError> for InitProjectError {
    fn from(e: FileWriteError) -> Self {
        todo!()
    }
}

impl From<MavenProjectError> for InitProjectError {
    fn from(e: MavenProjectError) -> Self {
        InitProjectError::MavenProject(e)
    }
}

impl From<AnsibleProjectError> for InitProjectError {
    fn from(e: AnsibleProjectError) -> Self {
        InitProjectError::AnsibleProject(e)
    }
}

impl fmt::Display for InitProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitProjectError::MavenProject(e) => {
                write!(f, "{}", e)
            }
            InitProjectError::AnsibleProject(e) => {
                write!(f, "{}", e)
            }
            _ => todo!(), // TODO: need exhaustive match arms
        }
    }
}

#[derive(Clone)]
pub struct ProjectSetting {
    pub key: String,
    pub val: String,
}

impl FromStr for ProjectSetting {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, v) = s.split_once('=').ok_or("Expected KEY=VALUE")?;
        Ok(Self {
            key: k.into(),
            val: v.into(),
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
    pub settings: Option<Vec<ProjectSetting>>,
}

struct ProjectInitializer<T: ProjectStrategy> {
    initialize_strategy: T,
}

impl<T: ProjectStrategy> ProjectInitializer<T> {
    fn new(initialize_strategy: T) -> Self {
        Self {
            initialize_strategy,
        }
    }

    fn initialize(&self, source: &Path) -> Result<(), InitProjectError> {
        self.initialize_strategy.write_templates(source)
    }
}

pub trait ProjectStrategy {
    fn write_templates(&self, source: &Path) -> Result<(), InitProjectError>;
}

struct ProjectFactory;

impl ProjectFactory {
    fn new(
        project_type: &str,
        settings: HashMap<String, String>,
    ) -> Result<Box<dyn ProjectStrategy>, InitProjectError> {
        match project_type {
            "MAVEN" => Ok(Box::new(MavenProject::new(settings)?)),
            "ANSIBLE" => Ok(Box::new(AnsibleProject::new(settings)?)),
            _ => Err(InitProjectError::Invalid("Unknown project type".into())),
        }
    }
}

pub fn handle(args: InitProjectArgs, config: AppConfig) -> Result<(), InitProjectError> {
    let template = config
        .templates
        .iter()
        .find(|p| {
            if let Some(profile) = &args.profile {
                p.name == args.project_type && p.profile == *profile
            } else {
                p.name == args.project_type && p.profile == "default"
            }
        })
        .ok_or(InitProjectError::Invalid("Could not find template".into()))?;

    // Convert custom key=value settings into map
    // for easier lookup
    let settings = match args.settings {
        Some(s) => s.into_iter().map(|kv| (kv.key, kv.val)).collect(),
        None => HashMap::new(),
    };

    match ProjectFactory::new(args.project_type.to_uppercase().as_str(), settings) {
        Ok(project_template) => {
            let _ = project_template.write_templates(template.template_files.as_path())?;
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
