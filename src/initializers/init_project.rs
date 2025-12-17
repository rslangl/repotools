//! src/initializers/init_project.rs

use std::{
    fmt,
    io,
    collections::HashMap, fs,
    path::{Path, PathBuf},
    str::FromStr};

use clap::Args;
use serde::Serialize;

use crate::initializers::project_types::maven::{MavenProject, MavenProjectError};
use crate::initializers::project_types::ansible::AnsibleProject;
use crate::app_config::app_config::AppConfig;

#[derive(Debug)]
pub enum InitProjectError {
    Io(io::Error),
    Render(tera::Error),
    Invalid(String),
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
    MavenProject(MavenProjectError)
    // TODO: same for ansible
}

impl From<io::Error> for InitProjectError {
    fn from(e: io::Error) -> Self {
        InitProjectError::Io(e)
    }
}

impl From<tera::Error> for InitProjectError {
    fn from(e: tera::Error) -> Self {
        InitProjectError::Render(e)
    }
}

impl From<MavenProjectError> for InitProjectError {
    fn from(e: MavenProjectError) -> Self {
        InitProjectError::MavenProject(e)
    }
}

impl fmt::Display for InitProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitProjectError::MavenProject(e) => {
                write!(f, "{}", e)
            },
            _ => todo!() // TODO: need exhaustive match arms
        }
    }
}

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


struct ProjectInitializer<T: ProjectStrategy> {
    initialize_strategy: T,
}

impl<T: ProjectStrategy> ProjectInitializer<T> {
    fn new(initialize_strategy: T) -> Self {
        Self { initialize_strategy }
    }

    fn initialize(&self, source: &Path) -> Result<(), InitProjectError> {
        self.initialize_strategy.write_templates(source)
    }
}

// Rendering templates with `Tera` require a value that implements `serde::Serializer`,
// and adding the `#[serde(untagged)]` directive tells `Serde` and `Tera` to serialize the
// enum as the contained value
#[derive(Serialize)]
#[serde(untagged)]
pub enum Val {
    Str(String),
    Num(i64),
    Bool(bool),
    Seq(Vec<Val>),
    Map(HashMap<String, Val>),
}

pub trait ProjectStrategy {
    fn write_templates(&self, source: &Path) -> Result<(), InitProjectError>;
}

struct ProjectFactory;

impl ProjectFactory {
    fn new(project_type: &str, settings: HashMap<String, String>) -> Result<Box<dyn ProjectStrategy>, InitProjectError> {
        match project_type {
            "MAVEN" => Ok(Box::new(MavenProject::new(settings)?)),
            "ANSIBLE" => Ok(Box::new(AnsibleProject::new(settings))),
            _ => return Err(InitProjectError::Invalid("Unknown project type".into())),
        }
    }
}

fn render(content: String, properties: &HashMap<String, Val>) -> Result<Vec<u8>, InitProjectError> {
    let mut context = tera::Context::new();

    for (key, val) in properties.iter() {
        context.insert(key.as_str(), val);
    }

    let rendered = match tera::Tera::one_off(&content, &context, false) {
        Ok(r) => {
            if r.is_empty() {
                return Err(InitProjectError::Invalid("Empty resource file".into()))
            }
            r
        },
        Err(e) => return Err(InitProjectError::Render(e.into()))
    };

    Ok(rendered.as_bytes().to_vec())
}

pub fn create_files(root: &Path, current: &Path, properties: &HashMap<String, Val>) -> Result<(), InitProjectError> {
    for entry in fs::read_dir(current).unwrap() {

        let entry = entry.unwrap();
        let path = entry.path();
        let relative_path = path.strip_prefix(root).unwrap();

        if path.is_dir() {
            let _ = create_files(root, &path, &properties);
            continue;
        }

        let target_root = Path::new(".");   // TODO: using current dir for now

        let target = target_root.join(relative_path);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| InitProjectError::Write {
                path: parent.to_path_buf(),
                source: e
            })?;
        }

        let content = fs::read_to_string(&path)?;

        let rendered = render(content, properties)?;
        fs::write(target, rendered).map_err(|e| InitProjectError::Write {
            path: path.clone(),
            source: e,
        })?;
    }

    Ok(())
}

pub fn handle(args: InitProjectArgs, config: AppConfig) -> Result<(), InitProjectError> {

    let template = config.templates.iter().find(|p| {
        if let Some(profile) = &args.profile {
            p.name == args.project_type && p.profile == *profile
        } else {
            p.name == args.project_type && p.profile == "default"
        }
    }).ok_or(InitProjectError::Invalid("Could not find template".into()))?;

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

    match ProjectFactory::new(args.project_type.to_uppercase().as_str(), settings) {
        Ok(project_template) => {
            let _ = project_template.write_templates(template.template_files.as_path())?;
        },
        Err(e) => return Err(e)
    }

    Ok(())
}
