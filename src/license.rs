use clap::{Arg, Command};
use std::path::PathBuf;
use lazy_static::lazy_static;

lazy_static! {
    static ref LICENSES: Option<Vec<License>> = {
        Some(vec![
            License::new("MIT"),
            License::new("GPLv3")
        ])
    };
}

#[derive(Clone)]
struct License {
    name: String,
    auto_fetch: bool,
    file_path: PathBuf,
    remote_url: String,
}

impl License {
    fn new(name: &str) -> License {
        License {
            name: String::from(name),
            auto_fetch: true,
            file_path: PathBuf::from(name),
            remote_url: String::from(name)
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

