use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::Serialize;
use crate::license::License;

#[derive(Serialize)]
pub struct Config {
    config_dir: PathBuf,
    data_dir: PathBuf,
    auto_fetch: bool,
    licenses: Vec<License>,
}

impl Config {
    pub fn new(cfg: PathBuf, data: PathBuf, licenses: Vec<License>) -> Config {
        Config{
            config_dir: cfg,
            data_dir: data,
            auto_fetch: true,
            licenses: licenses,

        }
    }

    pub fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    pub fn config_dir(&self) -> PathBuf {
        self.config_dir.clone()
    }
}

pub fn get_cfg() -> Result<Config, String> {    // TODO: use io::error::Error instead to avoid
                                                // mapping

    let base_dirs = xdg::BaseDirectories::with_prefix("repotools");

    if let Ok(base_dirs) = base_dirs {

        let data_dir = match base_dirs.create_data_directory("data") {
            Ok(d) => { 
                println!("Data directory created at: {:?}", d);
                d
            }
            Err(e) => {
                println!("Failed to create data directory: {}", e);
                PathBuf::new()
            }
        };

        let cfg_dir = match base_dirs.create_config_directory("config") {
            Ok(c) => {
                println!("Config directory created at: {:?}", c);
                c
            },
            Err(e) => {
                println!("Failed to create config directory: {}", e);
                PathBuf::new()
            }
        };

        let licenses = vec![License::new(
            &data_dir.to_str().unwrap(), 
            "MIT", 
            "https://raw.githubusercontent.com/aws/mit-0/refs/heads/master/MIT-0")];

        let config = Config::new(cfg_dir.clone(), data_dir, licenses);
        let toml = toml::to_string(&config).unwrap();

        let mut config_file = File::create(cfg_dir.join("config").to_str().unwrap()).map_err(|e| e.to_string())?;
        let _ = config_file.write_all(toml::to_string(&config).unwrap().as_bytes());

        println!("{}", toml);

        Ok(config)

    } else {
        eprintln!("Failed to get base directories");
        Err("Failed to get base directories".to_string())
    }
    //Ok(Config::new(base_dirs.config_dir().unwrap(), base_dirs.data_dir().unwrap()))
}
