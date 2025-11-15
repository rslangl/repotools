//! config/config.rs

static REPOTOOLS_DEFAULT_CONFIG_PATH: &str = "~/.config/repotools/config";

pub fn get_config(file_path: std::path::PathBuf) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {

    let config = config::Config::builder()
        .add_source(config::File::with_name("~./config/repotools/config"))
        .build()?
        .try_deserialize::<std::collections::HashMap<String, String>>()?;

    Ok(config)
}
