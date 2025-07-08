use clap::Command;

use config::Config;

use repotools::mgmt_license::{self, LicenseManager};
use repotools::mgmt_readme;

fn cli() -> Command {
    Command::new("")
        .about("repotools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            mgmt_license::get_cmd()
        )
        .subcommand(
            mgmt_readme::get_cmd()
        )
}

fn main() {

    let xdg_dirs = xdg::BaseDirectories::with_prefix("repotools");
    let config_file = xdg_dirs.find_config_file("config.toml").unwrap();

    let config_builder = Config::builder()
        .add_source(config::File::with_name(config_file.to_str().expect("Could not find config file")));

    let config = match config_builder.build() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error loading config file: {}", err);
            std::process::exit(1);
        }
    };

    let license_mgmt = LicenseManager::new();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("license", sub_matches)) => {
            println!("license: {}", sub_matches.get_one::<String>("LICENSE").expect("required"));
        },
        Some(("readme", sub_matches)) => {
            println!("readme: {}", sub_matches.get_one::<String>("PATH").expect("required"));
        },
        _ => unreachable!(),
    }
}
