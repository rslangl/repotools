
pub struct Config {
}

impl Config {
    pub fn new() -> Config {
        Config{}
    }
}

pub fn get_cfg() -> Result<Config, String> {

    let base_dirs = xdg::BaseDirectories::with_prefix("repotools");

    if let Ok(base_dirs) = base_dirs {

        match base_dirs.create_data_directory("data") {
            Ok(data_dir) => println!("Data directory created at: {:?}", data_dir),
            Err(e) => println!("Failed to create data directory: {}", e),
        }

        match base_dirs.create_config_directory("config") {
            Ok(cfg_dir) => println!("Config directory created at: {:?}", cfg_dir),
            Err(e) => println!("Failed to create config directory: {}", e),
        }
    } else {
        eprintln!("Failed to get base directories");
    }
    
    Ok(Config::new())
}
