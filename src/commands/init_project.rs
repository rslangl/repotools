//! commands/init_project.rs

use std::collections::HashMap;

use clap::Args;

#[derive(Args)]
pub struct InitProjectArgs {
    #[arg(long = "type")]
    pub project_type: String,
    #[arg(long)]
    pub profile: Option<String>
}

pub fn handle(args: InitProjectArgs, config: HashMap<String, String>) {

    // TODO: iterate list over registered profiles in config file and determine whether the passed
    // one exists. If not, terminate. If empty, use "default"

    println!("InitProject type: {}", &args.project_type);
    
}
