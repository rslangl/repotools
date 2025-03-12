use std::path::PathBuf;

pub struct Config {
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl Config {
    pub fn new(cfg: PathBuf, data: PathBuf) -> Config {
        Config{
            config_dir: cfg,
            data_dir: data
        }
    }

    pub fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    pub fn config_dir(&self) -> PathBuf {
        self.config_dir.clone()
    }
}

pub fn get_cfg() -> Result<Config, String> {

    let base_dirs = xdg::BaseDirectories::with_prefix("repotools");

    if let Ok(base_dirs) = base_dirs {

        let data = match base_dirs.create_data_directory("data") {
            Ok(data_dir) => { 
                println!("Data directory created at: {:?}", data_dir);
                data_dir
            }
            Err(e) => {
               println!("Failed to create data directory: {}", e);
                PathBuf::new()
            }
        };

        let cfg = match base_dirs.create_config_directory("config") {
            Ok(cfg_dir) => {
                println!("Config directory created at: {:?}", cfg_dir);
                cfg_dir
            },
            Err(e) => {
                println!("Failed to create config directory: {}", e);
                PathBuf::new()
            }
        };

        Ok(Config::new(cfg, data))

    } else {
        eprintln!("Failed to get base directories");
        Err("Failed to get base directories".to_string())
    }
    //Ok(Config::new(base_dirs.config_dir().unwrap(), base_dirs.data_dir().unwrap()))
}
