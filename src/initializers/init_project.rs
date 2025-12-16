//! src/initializers/init_project.rs

use std::{
    collections::HashMap, fs::self,
    path::Path,
    str::FromStr};

use clap::Args;
use serde::Serialize;

use crate::initializers::project_maven::MavenProject;
use crate::initializers::project_ansible::AnsibleProject;
use crate::app_config::app_config::AppConfig;

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


struct ProjectInitializer<T: ProjectStrategy> {
    initialize_strategy: T,
}

impl<T: ProjectStrategy> ProjectInitializer<T> {
    fn new(initialize_strategy: T) -> Self {
        Self { initialize_strategy }
    }

    fn initialize(&self, source: &Path) -> Result<(), String> {
        self.initialize_strategy.write_templates(source)
    }
}

/**
Rendering templates with `Tera` require a value that implements `serde::Serializer`,
and adding the `#[serde(untagged)]` directive tells `Serde` and `Tera` to serialize the
enum as the contained value
*/
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
    fn write_templates(&self, source: &Path) -> Result<(), String>;
}

struct ProjectFactory;

impl ProjectFactory {
    fn new(project_type: &str, settings: HashMap<String, String>) -> Result<Box<dyn ProjectStrategy>, String> {
        match project_type {
            PROJECT_TYPE_MAVEN => Ok(Box::new(MavenProject::new(settings))),
            PROJECT_TYPE_ANSIBLE => Ok(Box::new(AnsibleProject::new(settings))),
            _ => Err("Unknown strategy".to_string()),
        }
    }
}

fn render(content: String, properties: &HashMap<String, Val>) -> Vec<u8> {
    let mut context = tera::Context::new();

    for (key, val) in properties.iter() {
        context.insert(key.as_str(), val);
    }

    let rendered = tera::Tera::one_off(&content, &context, false).unwrap();
    rendered.as_bytes().to_vec()
}

pub fn create_files(root: &Path, current: &Path, properties: &HashMap<String, Val>) {
    for entry in fs::read_dir(current).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let relative_path = path.strip_prefix(root).unwrap();

        if path.is_dir() {
            create_files(root, &path, &properties);
            continue;
        }

        let target_root = Path::new(".");   // TODO: using current dir for now
        let target = target_root.join(relative_path);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let content = fs::read_to_string(&path).unwrap();
        let rendered = render(content, properties);
        fs::write(target, rendered).unwrap();
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
        let _ = project.write_templates(template.template_files.as_path());
    }

    Ok(())
}
