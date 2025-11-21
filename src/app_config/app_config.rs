//! config/config.rs

use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use config::FileFormat;
use tera::{Tera, Context};

const DEFAULT_CONFIG: &'static str = "
auto_fetch=true

[[licenses]]
name=\"MIT\"
file_path=\"{{ data_dir }}MIT\"
remote_src=\"https://raw.githubusercontent.com/aws/mit-0/refs/heads/master/MIT-0\"
";

pub fn get_config(file_path: Option<String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {

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
        .try_deserialize::<HashMap<String, String>>()?;

    Ok(config)
}
