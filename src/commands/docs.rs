//! commands/docs.rs

#[derive(Args)]
pub struct DocsArgs {
    #[arg(long, value_delimiter = ',')]
    pub name: String,

    #[arg(long)]
    pub overwrite: bool,
}

pub fn handle(args: &DocsArgs) {

    if let Some(name) = &args.name {
        println!("Docs: {}", name);
    }

    if let Some(overwrite) = &args.name {
        println!("Overwrite: {}", overwrite);
    }
}
