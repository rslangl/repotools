use clap::{arg, Command};

mod license;
mod readme;

fn main() {
    let cmd = Command::new("")
        .about("repotools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            license::get_cmd()
            )
        .subcommand(
            readme::get_cmd()
            );

    let matches = cmd.get_matches();

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
