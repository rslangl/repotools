//! config/config.rs

use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use config::{Config, FileFormat};
use reqwest::Url;
use serde::Deserialize;
use tera::{Tera, Context};

#[derive(Deserialize)]
pub struct AppConfig {
    pub auto_fetch: bool,
    pub licenses: Vec<License>,
    pub templates: Vec<ProjectTemplate>,
}

#[derive(Deserialize)]
struct License {
    name: String,
    file_path: PathBuf,
    remote_src: Url
}

#[derive(Deserialize)]
pub struct ProjectTemplate {
    pub name: String,
    pub profile: String,
    pub template_files: PathBuf
}

pub fn get_config(file_path: Option<String>) -> Result<AppConfig, Box<dyn std::error::Error>> {

    let config_path = match file_path {
        Some(path) => PathBuf::from(path),
        None => {
            let xdg = xdg::BaseDirectories::with_prefix("repotools");
            let path = xdg.place_config_file("config").expect("Could not create config directory");

            if !path.exists() {
                let default_config = include_str!("../../assets/config.toml.j2");
                let mut context = tera::Context::new();
                context.insert("data_dir", &xdg.get_cache_home().expect("Could not get cache directory"));
                let rendered = Tera::one_off(default_config, &context, false).unwrap();
                let mut f = fs::File::create(&path).unwrap();
                f.write_all(rendered.as_bytes()).unwrap();
            }

            path
        }
    };

    let config = Config::builder()
        .add_source(config::File::new(config_path.to_str().unwrap(), FileFormat::Toml))
        .build()?
        .try_deserialize::<AppConfig>()?;

    Ok(config)
}
