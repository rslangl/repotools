//! src/cli.rs

use clap::{Parser, Subcommand};
use crate::commands::{DocsArgs, GlobalOpts, LicenseArgs, LinterArgs};

#[derive(Parser)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Docs(DocsArgs),
    License(LicenseArgs),
    Linter(LinterArgs),
}
