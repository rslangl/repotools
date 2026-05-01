use clap::{Args, Parser, ValueEnum};

use std::{fmt, fmt::Write};

use crate::{
    app_config::AppConfig,
};

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ListItem {
    All,
    Templates,
    Features,
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListItem::All => {
                write!(f, "all")
            }
            ListItem::Templates => {
                write!(f, "templates")
            }
            ListItem::Features => {
                write!(f, "features")
            }
        }
    }
}

#[derive(Args)]
pub struct ListItemArgs {

    #[arg(short, long, default_value_t = ListItem::All)]
    pub select: ListItem,
}

pub fn list_items(args: ListItemArgs, config: AppConfig) -> Result<(), String> {

    let mut out = String::new();

    // Gets all available projects that is configured
    if args.select == ListItem::Templates || args.select == ListItem::All {
        write!(out, "Project templates:\n------------------\n");
        let templates = config
            .templates
            .iter()
            .for_each(|t|
                write!(out, "Type: {}\nProfile: {}\n\n", t.name, t.profile).unwrap()
            );
    }
    
    // Gets all available features that are configured
    if args.select == ListItem::Features || args.select == ListItem::All {
        write!(out, "Feature: Licenses:\n------------------\n");
        let features = config
            .features
            .licenses
            .iter()
            .for_each(|t|
                write!(out, "Name: {}\n\n", t.name).unwrap()
            );

        write!(out, "Feature: Linters:\n-----------------\n");
        let linters = config
            .features
            .linters
            .iter()
            .for_each(|t|
                write!(out, "Type: {}\n\n", t.name).unwrap()
            );
    }

    // Get all available linters that i
    println!("{}", out);

    Ok(())
}
