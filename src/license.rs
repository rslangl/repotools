use clap::{arg, Command};

pub fn get_cmd() -> clap::Command {
    clap::Command::new("license")
        .about("license")
        .arg(clap::arg!(<LICENSE> "The license to add"))
        .arg_required_else_help(true)
}
