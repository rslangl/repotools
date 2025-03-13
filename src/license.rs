use clap::{Arg, Command};
use std::path::PathBuf;
//use lazy_static::lazy_static;
//use std::sync::Mutex;
use serde::Serialize;
//use crate::config::Config;

//lazy_static! {
//    static ref LICENSES: Mutex<Option<Vec<License>>> = Mutex::new(
//        Some(vec![
//            License::new("MIT", "https://raw.githubusercontent.com/aws/mit-0/refs/heads/master/MIT-0"),
//            License::new("GPLv3")
//        ])
//    );
//}

#[derive(Serialize)]
pub struct License {
    name: String,  
    file_path: PathBuf,
    remote_url: String,
}

impl License {
    pub fn new(path: &str, name: &str, url: &str) -> License {
        License {
            name: String::from(name),
            file_path: PathBuf::from(path).join(name),
            remote_url: String::from(url)
        }
    }

//    pub fn init(cfg: Config) {
//
//        let mut guard = LICENSES.lock().unwrap();
//
//        if let Some(lic) = guard.as_mut() {
//            for l in lic.iter_mut() {
//                l.file_path = cfg.data_dir();
//                let uri = l.remote_url.parse()?;
//                let f = cfg.data_dir().push(l.name);
//            }
//        }
//    }
}

pub fn get_cmd() -> clap::Command {
    clap::Command::new("license")
        .about("license")
        .arg(
            Arg::new("LICENSE")
            .required(true),
        )
        .arg_required_else_help(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Command, Arg};

    fn mock_cli() -> Command {
        Command::new("addlicense")
            .subcommand(
                get_cmd()
            )
    }

    #[test]
    fn valid_license() {
        let matches = mock_cli().get_matches_from(vec![
            "addlicense", "license", "MIT"
        ]);

        if let Some(license_matches) = matches.subcommand_matches("license") {
            let license_value = license_matches.get_one::<String>("name").unwrap();
            assert!(LICENSES.clone().unwrap().iter().any(|item| item.name == *license_value));
        }
    }

    #[test]
    fn invalid_license() {
        let matches = mock_cli().get_matches_from(vec![
            "addlicense", "license", "NotALicense"
        ]);

        if let Some(license_matches) = matches.subcommand_matches("license") {
            let license_value = license_matches.get_one::<String>("name").unwrap();
            assert!(!LICENSES.clone().unwrap().iter().any(|item| item.name == *license_value));
        }
    }
}

