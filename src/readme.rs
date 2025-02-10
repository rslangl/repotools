use clap::{Arg, Command};
use lazy_static::lazy_static;
use tera::{Tera, Context};
use std::path::PathBuf;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

lazy_static! {

    // Template contents
    static ref CONTENT: HashMap<&'static str, &'static str> = HashMap::from([
        ("overview", "some overview intro content"),
        ("usage", "some usage content")
    ]);

    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("../templates/readme/template/readme.md") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        //tera.autoescape_on(vec!["md"]);
        tera
    };
}

struct Readme {
    file_path: PathBuf,
    overview: String,
    usage: String
}

impl Readme {

    fn new(path: PathBuf, content: HashMap<&'static str, &'static str>) {
        let mut context = Context::new();

        context.insert("overview", &content.get(&"overview"));
        context.insert("usage", &content.get(&"usage"));

        Tera::one_off("overview", &Context::new(), true).unwrap();

        match TEMPLATES.render("readme.md", &context) {
            Ok(s) => {

                // Create the actual file on disk
                let mut readme_file = match File::create(&path) {
                    Err(e) => panic!("Could not create file: {}", e),
                    Ok(f) => f,
                };

                match readme_file.write_all(s.as_bytes()) {
                    Err(e) => panic!("Could not write to file: {}", e),
                    Ok(_) => println!("Wrote contents to file")
                };
            },
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
            }
        };
    }
}

pub fn get_cmd() -> clap::Command {
    clap::Command::new("readme")
        .about("Adds or manages README files")
        //.arg(clap::arg!(<PATH> "Path"))
        .arg(
            Arg::new("path")
            .short('p')
            .long("path")
            )
}

#[cfg(test)]
mod tests {
    use super::Readme;
    use crate::readme::get_cmd;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use clap::{Command, Arg};

    fn mock_cli() -> Command {
        Command::new("readme")
            .subcommand(
                get_cmd()
            )
    }

    #[test]
    fn validate_generated_readme_with_path_arg() {
        let CONTENT: HashMap<&'static str, &'static str> = HashMap::from([
            ("intro", "some intro content"),
            ("usage", "some usage content")
        ]);

        let matches = mock_cli().get_matches_from(vec![
            "readme", "-p", "."
        ]);

        // TODO: check file contents and path correctness
        if let Some(readme_matches) = matches.subcommand_matches("readme") {
            let readme_path = readme_matches.get_one::<String>("--path").unwrap();
            let readme_file = Readme::new(PathBuf::from(readme_path), CONTENT);
            
        }

    }

    #[test]
    fn validate_generated_readme_without_path_arg() {
        // TODO: check file contents and path correctness
    }
}
