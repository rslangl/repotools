// mgmt_repo.rs

use clap::{Arg, Command};

pub fn get_cmd() -> clap::Command {
    clap::Command::new("repo")
        .about("repo management")
        .arg(
            Arg::new("init")
            .required(true),
        )
        .arg_required_else_help(true)
}
