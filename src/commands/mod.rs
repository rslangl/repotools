//! src/commands/mod.rs

pub mod docs;
pub mod license;
pub mod linter;

pub use docs::{DocsArgs, handle as docs_handle};
pub use license::{LicenseArgs, handle as license_handle};
pub use linter::{LinterArgs, handle as linter_handle};

use clap::Args;

#[derive(Args)]
pub struct GlobalOpts {
    #[arg(long)]
    pub config: Option<String>,
}
