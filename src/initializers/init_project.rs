//! src/initializers/init_project.rs

use std::{collections::HashMap, fmt, path::PathBuf, str::FromStr};

use clap::Args;

use crate::{
    app_config::{AppCache, app_config::AppConfig},
    initializers::project_types::{
        ansible::{AnsibleProject, AnsibleProjectError},
        maven::{MavenProject, MavenProjectError},
    },
    utils::file_writer::FileWriteError,
};

#[derive(Debug)]
pub enum InitProjectError {
    Invalid(String),
    FileWrite(FileWriteError),
    NotFound(PathBuf),
    // Specific project type errors
    MavenProject(MavenProjectError),
    AnsibleProject(AnsibleProjectError),
}

impl From<FileWriteError> for InitProjectError {
    fn from(e: FileWriteError) -> Self {
        InitProjectError::FileWrite(e)
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
            InitProjectError::Invalid(e) => {
                write!(f, "{}", e)
            }
            InitProjectError::FileWrite(e) => {
                write!(f, "{}", e)
            }
            InitProjectError::NotFound(e) => {
                write!(f, "Template files not found: {}", e.display())
            }
            InitProjectError::MavenProject(e) => {
                write!(f, "{}", e)
            }
            InitProjectError::AnsibleProject(e) => {
                write!(f, "{}", e)
            }
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

struct ProjectInitializer {
    initialize_strategy: Box<dyn ProjectStrategy>,
}

impl ProjectInitializer {
    fn new(initialize_strategy: Box<dyn ProjectStrategy>) -> Self {
        Self {
            initialize_strategy,
        }
    }

    fn initialize(self) -> Result<(), InitProjectError> {
        Ok(self.initialize_strategy.write_templates()?)
    }
}

pub trait ProjectStrategy {
    fn write_templates(self: Box<Self>) -> Result<(), InitProjectError>;
}

struct ProjectFactory;

impl ProjectFactory {
    fn new(
        project_type: String,
        template_files: PathBuf,
        settings: HashMap<String, String>,
    ) -> Result<Box<dyn ProjectStrategy>, InitProjectError> {
        match project_type.to_uppercase().as_str() {
            "MAVEN" => Ok(Box::new(MavenProject::new(template_files, settings)?)),
            "ANSIBLE" => Ok(Box::new(AnsibleProject::new(template_files, settings)?)),
            _ => Err(InitProjectError::Invalid("Unknown project type".into())),
        }
    }
}

pub fn handle(
    args: InitProjectArgs,
    config: AppConfig,
    cache: AppCache,
) -> Result<(), InitProjectError> {
    // Ensure the passed project type and given profile, if any, is present in the config file
    // before passing it along
    let template: PathBuf = config
        .templates
        .iter()
        .find(|p| {
            if let Some(profile) = &args.profile {
                p.name == args.project_type && p.profile == *profile
            } else {
                p.name == args.project_type && p.profile == "default"
            }
        })
        .ok_or(InitProjectError::Invalid(String::from(
            "Could not find template",
        )))
        .and_then(|p| -> Result<PathBuf, InitProjectError> {
            println!("CACHE DIR: {}", cache.cache_dir.clone().display());
            if p.template_files.starts_with(cache.cache_dir) {
                Ok(p.template_files.clone())
            } else {
                Err(InitProjectError::NotFound(p.template_files.clone()))
            }
        })?;

    // Convert custom key=value settings into map
    // for easier lookup
    let settings = match args.settings {
        Some(s) => s.into_iter().map(|kv| (kv.key, kv.val)).collect(),
        None => HashMap::new(),
    };

    let strategy: Box<dyn ProjectStrategy> =
        ProjectFactory::new(args.project_type, template, settings)?;

    let initializer: ProjectInitializer = ProjectInitializer::new(strategy);

    initializer.initialize()?;

    Ok(())
}
