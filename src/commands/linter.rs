//! commands/linter.rs

#[derive(Args)]
pub struct LinterArgs {
    #[arg(long, value_delimiter = ',')]
    pub name: String,

    #[arg(long)]
    pub overwrite: bool,
}

pub fn handle(args: &LinterArgs) {

    if let Some(name) = &args.name {
        println!("Linter: {}", name);
    }

    if let Some(overwrite) = &args.overwrite {
        println!("Overwrite: {}", overwrite);
    }
}
