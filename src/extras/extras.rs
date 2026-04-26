use clap::{Args, Parser, ValueEnum};

use std::fmt;

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

    // Prints all available projects that is configured
    config
        .templates
        .iter()
        .for_each(|t|
            println!("{}", t.name)
        );

    Ok(())
}
