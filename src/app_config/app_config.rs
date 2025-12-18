//! src/app_config/config.rs

use std::{fs, io::{self, Write}, path::PathBuf};
use config::{Config, FileFormat};
use reqwest::Url;
use serde::Deserialize;
use tera::Tera;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Load(config::ConfigError),
    Render(tera::Error),
    Invalid(String),
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<config::ConfigError> for ConfigError {
    fn from(e: config::ConfigError) -> Self {
        ConfigError::Load(e)
    }
}

impl From<tera::Error> for ConfigError {
    fn from(e: tera::Error) -> Self {
        ConfigError::Render(e)
    }
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub auto_fetch: bool,
    pub licenses: Vec<License>,
    pub templates: Vec<ProjectTemplate>,
}

// TODO: add other structs reflecting the contents of the resources found in src/resources
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

pub fn get_config(file_path: Option<String>) -> Result<AppConfig, ConfigError> {

    let config_path = match file_path {
        Some(path) => PathBuf::from(path),
        None => {
            let xdg = xdg::BaseDirectories::with_prefix("repotools");
            let path = xdg.place_config_file("config")?;

            if !path.exists() {
                let default_config = include_str!("../../assets/config.toml.j2");

                let mut context = tera::Context::new();
                context.insert("data_dir", &xdg.get_cache_home());

                let rendered = match Tera::one_off(default_config, &context, false) {
                    Ok(r) => {
                        if r.is_empty() {
                            return Err(ConfigError::Invalid("Empty config file output".into()));
                        }
                        r
                    }
                    Err(e) => return Err(ConfigError::Render(e.into()))
                };

                let mut f = fs::File::create(&path)?;
                f.write_all(rendered.as_bytes()).map_err(|e| ConfigError::Write {
                    path: path.clone(),
                    source: e,
                })?;
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
