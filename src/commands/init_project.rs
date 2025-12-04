//! commands/init_project.rs

use std::{collections::HashMap, fs::{self, File, ReadDir}, io::{Read, Write}, path::{Path, PathBuf}, str::FromStr};

use clap::{Args, Error};
use serde::Serialize;
use tera::Tera;

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

#[derive(Serialize)]
enum Val {
    Str(String),
    Num(i64),
    Bool(bool),
    Seq(Vec<Val>),
    Map(HashMap<String, Val>),
}

trait ProjectStrategy {
    fn write_templates(&self, source: &Path) -> Result<(), String>;
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

    fn get_properties(&self) -> HashMap<String, Val> {
        let mut properties = HashMap::new();
        properties.insert("group_id".to_string(), Val::Str(self.group_id.clone()));
        properties.insert("artifact_id".to_string(), Val::Str(self.artifact_id.clone()));
        properties
    }
}

impl ProjectStrategy for MavenProject {

    fn write_templates(&self, source: &Path) -> Result<(), String> {
        create_files(&source, &source, &MavenProject::get_properties(self));
        Ok(())
    }
}

struct AnsibleProject {
    // Host tuples are FQDN/hostnames and IP
    hosts: Vec<String>,
    roles: Vec<String>
}

impl AnsibleProject {
    fn new(settings: HashMap<String, String>) -> Self {

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

fn render(content: String, properties: &HashMap<String, Val>) -> Vec<u8> { // TODO: pass project-specific values
    let mut context = tera::Context::new();

    for (key, val) in properties.iter() {
        match val {
            Val::Str(s) => context.insert(key.as_str(), s),
            Val::Bool(b) => context.insert(key.as_str(), b),
            Val::Seq(xs) => context.insert(key.as_str(), xs),
            Val::Num(n) => context.insert(key.as_str(), n),
            Val::Map(m) => context.insert(key.as_str(), m),
        }
        //context.insert(key.as_str(), val.as_str());
    }

    let rendered = tera::Tera::one_off(&content, &context, false).unwrap();
    rendered.as_bytes().to_vec()
}

fn create_files(root: &Path, current: &Path, properties: &HashMap<String, Val>) {
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
        let rendered = render(content, properties.clone());
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
        project.write_templates(template.template_files.as_path());
    }

    Ok(())
}
