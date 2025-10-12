//! commands/docs.rs

use clap::Args;

#[derive(Args)]
pub struct DocsArgs {
    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub overwrite: bool,
}

pub fn handle(args: DocsArgs) {
    println!("Docs: {}", &args.name);
    println!("Overwrite: {}", &args.overwrite);
}
