//! src/commands/mod.rs

pub mod init_project;

pub use init_project::{InitProjectArgs, handle as init_project_handler};

use clap::Args;

#[derive(Args)]
pub struct GlobalOpts {
    #[arg(long, name = "config", default_value = "~/.config/repotools/config")]
    pub config_path: String,
}
