use clap::{Arg, Command};
use std::path::PathBuf;
use serde::Serialize;
use crate::http_util::client::HttpClient;

pub struct LicenseManager {
    http_client: HttpClient,
}

impl LicenseManager {
    pub fn new(http_client: HttpClient) -> Self {
        LicenseManager {
            http_client
        }
    }
    pub async fn download_resource(&self, url: String) -> Result<String, String> {
        match self.http_client.exec(url) {
            Ok(_) => Ok(String::from("Downloaded successfully")),
            Err(e) => Err(e.to_string())
        }
    }
}

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

