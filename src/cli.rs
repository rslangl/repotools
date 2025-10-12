//! cli.rs

use clap::{Parser, Subcommand};

pub mod commands;

#[derive(Parser)]
#[command(name = "repotools", author, version)]
pub struct Cli {
    #[command(flatten)]
    pub global: commands::GlobalOpts,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    License(commands::license::LicenseArgs),
    Docs(commands::docs::DocsArgs),
    Linter(commands::linter::LinterArgs),
}
