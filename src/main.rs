mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

fn main() {

    // let xdg_dirs = xdg::BaseDirectories::with_prefix("repotools");
    // let config_file = xdg_dirs.find_config_file("config.toml").expect("Could not find config.toml");
    //
    // let config_builder = Config::builder()
    //     .add_source(config::File::with_name(config_file.to_str().expect("Could not find config file")));
    //
    // let config = match config_builder.build() {
    //     Ok(config) => config,
    //     Err(err) => {
    //         eprintln!("Error loading config file: {}", err);
    //         std::process::exit(1);
    //     }
    // };
    
    let cli = Cli::parse();

    match cli.command {
        Command::Docs(args) => commands::docs::handle(args),
        Command::License(args) => commands::license::handle(args),   // for global args, add &cli.global
        Command::Linter(args) => commands::linter::handle(args),
    }
}
