//! config/config.rs

use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use config::FileFormat;
use reqwest::Url;
use serde::Deserialize;
use tera::{Tera, Context};

const DEFAULT_CONFIG: &'static str = "
auto_fetch=true

[[licenses]]
name=\"MIT\"
file_path=\"{{ data_dir }}MIT\"
remote_src=\"https://raw.githubusercontent.com/aws/mit-0/refs/heads/master/MIT-0\"

[[licenses]]
name=\"LGPL\"
file_path=\"{{ data_dir }}LGPL\"
remote_src=\"https://raw.githubusercontent.com/git/git/refs/heads/master/LGPL-2.1\"

[[templates]]
name=\"maven\"
profile=\"default\"
template_files=\"{{ data_dir }}maven/default/\"
";

const DEFAULT_TEMPLATE_MAVEN: &'static str = "
    <project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"
    xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd\">
    <modelVersion>4.0.0</modelVersion>

    <groupId>{{ mvn_group_id }}</groupId>
    <artifactId>{{ mvn_app_name }}</artifactId>
    <version>1.0-SNAPSHOT</version>

    </project>
";

#[derive(Deserialize)]
struct Config {
    auto_fetch: bool,
    licenses: Vec<License>,
    templates: Vec<ProjectTemplate>,
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

pub fn get_config(file_path: Option<String>) -> Result<HashMap<String, toml::Value>, Box<dyn std::error::Error>> {

    let config_path = match file_path {
        Some(path) => PathBuf::from(path),
        None => {
            let xdg = xdg::BaseDirectories::with_prefix("repotools");
            let path = xdg.place_config_file("config").expect("Could not create config directory");

            if !path.exists() {
                let mut context = tera::Context::new();
                context.insert("data_dir", &xdg.get_cache_home().expect("Could not get cache directory"));
                let rendered = Tera::one_off(DEFAULT_CONFIG, &context, false).unwrap();
                let mut f = fs::File::create(&path).unwrap();
                f.write_all(rendered.as_bytes()).unwrap();
            }

            path
        }
    };

    let config = config::Config::builder()
        .add_source(config::File::new(config_path.to_str().unwrap(), FileFormat::Toml))
        .build()?
        .try_deserialize::<HashMap<String, toml::Value>>()?;

    Ok(config)
}
