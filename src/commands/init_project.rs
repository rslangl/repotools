//! commands/init_project.rs

use std::{collections::HashMap, fs::{self, File}, io::{Read, Write}};

use clap::Args;

use crate::app_config::app_config::ProjectTemplate;

#[derive(Args)]
pub struct InitProjectArgs {
    #[arg(long = "type")]
    pub project_type: String,
    #[arg(long)]
    pub profile: Option<String>
}

pub fn handle(args: InitProjectArgs, config: HashMap<String, toml::Value>) {

    let serialized: &toml::Value = config.get("templates").expect("Could not find expected field 'templates' in config file");
    let templates: Vec<ProjectTemplate> = serialized.clone().try_into().expect("Could not deserialize list of templates from config file");
    let template = templates.iter().find(|p| {
        if let Some(profile) = &args.profile {
            p.name == args.project_type && p.profile == *profile
        } else {
            p.name == args.project_type && p.profile == "default"
        }
    }).expect("Could not find template");

    println!("name: {}", template.name);
    // println!("profile: {}", template.profile);
    // println!("files path: {}", template.template_files.display());

for template_file in fs::read_dir(template.template_files.as_path()).expect("Could not read files from template files path") {
    let source_template = template_file.unwrap();
    let source_file_name = source_template.file_name();
    let source_file_path = source_template.path();

    let mut source = File::open(source_file_path).unwrap();
    let mut content = String::new();
    source.read_to_string(&mut content).unwrap();

    let mut target_file = File::create(source_file_name).unwrap();
    target_file.write_all(content.as_bytes()).unwrap();
}
}
