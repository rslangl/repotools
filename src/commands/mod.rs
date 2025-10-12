//! commands/mod.rs

pub mod license;
pub mod docs;
pub mod linter;

pub use license::{LicenseArgs, handle};
pub use docs::{DocsArgs, handle};
pub use linter::{LinterArgs, handle};

use clap::Args;

#[derive(Args)]
pub struct GlobalOpts {
    #[arg(long)]
    pub config: Option<String>,
}
