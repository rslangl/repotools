mod license_management;
mod readme_management;
mod http_util;

mod config;

use crate::license::LicenseManager;
use crate::license_management::license;
use crate::readme_management::readme;

use clap::Command;

fn cli() -> Command {
    Command::new("")
        .about("repotools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            license::get_cmd()
        )
        .subcommand(
            readme::get_cmd()
        )
}

fn main() {

    let cfg = config::get_cfg().expect("config required");

    let license_service = LicenseManager::new();

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
