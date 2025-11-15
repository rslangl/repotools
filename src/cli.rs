//! src/cli.rs

use clap::{Parser, Subcommand};
use crate::commands::{GlobalOpts, InitProjectArgs};

#[derive(Parser)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(name = "init", about = "Initialize the project")]
    InitProject(InitProjectArgs),
    // TODO: add more subcommands as needed, e.g.
    // License: add or remove license, optional inlined
}
