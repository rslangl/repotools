//! commands/linter.rs

use clap::Args;

#[derive(Args)]
pub struct LinterArgs {
    #[arg(long, value_delimiter = ',')]
    pub name: Vec<String>,

    #[arg(long)]
    pub overwrite: bool,
}

pub fn handle(args: LinterArgs) {

    println!("Linter: {:?}", &args.name);
    println!("Overwrite: {}", &args.overwrite);
}
