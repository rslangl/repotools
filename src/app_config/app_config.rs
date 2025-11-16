//! config/config.rs

use std::{collections::HashMap, fs, io::Write, path::PathBuf};

pub fn get_config(file_path: Option<String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {

    let config_path = match file_path {
        Some(path) => PathBuf::from(path),
        None => {
            let xdg = xdg::BaseDirectories::with_prefix("repotools");
            let path = xdg.place_config_file("config").expect("Could not create config directory");

            if !path.exists() {
                let mut f = fs::File::create(&path).unwrap();
                f.write_all(b"# initial config\n").unwrap();
            }

            path
        }
    };

    let config = config::Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()?
        .try_deserialize::<HashMap<String, String>>()?;

    Ok(config)
}
