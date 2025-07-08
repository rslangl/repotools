use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::mgmt_license::License;

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct Config {
    auto_fetch: bool,
    licenses: Vec<License>,
}

// impl Config {
//     pub fn default(cfg: PathBuf, data: PathBuf) -> Config {
//
//         // TODO: add defaults to license_management crate
//         let licenses = vec![License::new(
//             data.join("MIT").to_str().unwrap(),
//             "MIT", 
//             "https://raw.githubusercontent.com/aws/mit-0/refs/heads/master/MIT-0")];
//
//         Config{
//             config_dir: cfg,
//             data_dir: data,
//             auto_fetch: true,
//             licenses: licenses,
//         }
//     }
//
//     pub fn data_dir(&self) -> PathBuf {
//         self.data_dir.clone()
//     }
//
//     pub fn config_dir(&self) -> PathBuf {
//         self.config_dir.clone()
//     }
// }
//
// pub fn get_cfg() -> Result<Config, String> {
//
//     let base = match xdg::BaseDirectories::with_prefix("repotools") {
//         Ok(b) => b,
//         Err(e) => {
//             return Err(format!("Failed to get application base directory"))
//         }
//     };
//
//
//     let cfg_path = match base.place_config_file(PathBuf::from("config")) {
//         Ok(path) => {
//             let data_dir = base.get_data_home().join("repotools");
//             let cfg_default = Config::default(path.clone(), data_dir);
//             if !path.exists() {
//                 fs::write(path.clone(), toml::to_string(&cfg_default).unwrap()).unwrap();//.expect("Failed to write default config")
//             }
//             path
//         },
//         Err(e) => {
//             return Err(format!("Failed to create config file"))
//         }
//     };
//
//     let config = match fs::read_to_string(cfg_path) {
//         Ok(c) => {
//             let cfg: Config = toml::from_str(&c).unwrap();
//             cfg
//         },
//         Err(e) => {
//             return Err(format!("Failed to parse config file"))
//         }
//     };
//
//     //println!("{}", config);
//
//     Ok(config)
// }
