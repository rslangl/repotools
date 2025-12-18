//! src/cli/cli.rs

use clap::{Parser, Subcommand, Args};

use crate::initializers::InitProjectArgs;
use crate::features::ProjectFeatureArgs;

#[derive(Parser)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args)]
pub struct GlobalOpts {

    #[arg(long, name = "config")]
    pub config_path: Option<String>,
    // TODO: add data/cache path
}

#[derive(Subcommand)]
pub enum Command {

    #[clap(name = "init", about = "Initialize the project")]
    InitProject(InitProjectArgs),

    #[clap(name = "feature", about = "Add a feature to the project")]
    ProjectFeature(ProjectFeatureArgs)
}

