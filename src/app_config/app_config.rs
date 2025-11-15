//! config/config.rs

pub fn get_config(file_path: std::path::PathBuf) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {

    let config = config::Config::builder()
        .add_source(config::File::with_name(file_path.to_str().expect("Invalid config file path")))
        .build()?
        .try_deserialize::<std::collections::HashMap<String, String>>()?;

    Ok(config)
}
