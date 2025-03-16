use crate::config::{Config, get_cfg};
use crate::license::{License, LicenseManager};
use crate::readme::get_cmd;
use crate::http_util::client::HttpClient;
use clap::Command;

mod license;
mod readme;
mod config;
mod http_util;

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

    let cfg = get_cfg().expect("config required");

    let http_client = Box::new(HttpClient::new());
    let license_service = LicenseManager { http_client };

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("license", sub_matches)) => {
            println!("license: {}", sub_matches.get_one::<String>("LICENSE").expect("required"));
            //License::init(cfg);
        },
        Some(("readme", sub_matches)) => {
            println!("readme: {}", sub_matches.get_one::<String>("PATH").expect("required"));
        },
        _ => unreachable!(),
    }
}
