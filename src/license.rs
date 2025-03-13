use clap::{Arg, Command};
use std::path::PathBuf;
//use lazy_static::lazy_static;
//use std::sync::Mutex;
use serde::Serialize;
//use crate::config::Config;
use http_body_util::{Empty, BodyExt};
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt as _};
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
//lazy_static! {
//    static ref LICENSES: Mutex<Option<Vec<License>>> = Mutex::new(
//        Some(vec![
//            License::new("MIT", "https://raw.githubusercontent.com/aws/mit-0/refs/heads/master/MIT-0"),
//            License::new("GPLv3")
//        ])
//    );
//}

#[derive(Serialize)]
pub struct License {
    name: String,  
    file_path: PathBuf,
    remote_url: String,
}

impl License {
    pub fn new(path: &str, name: &str, url: &str) -> License {

        License::download(url);

        License {
            name: String::from(name),
            file_path: PathBuf::from(path).join(name),
            remote_url: String::from(url)
        }
    }

    async fn download(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {   // TODO: move to separate util crate or summat
        //let url = self.remote_url.parse::<hyper::Uri>()?;
        let url = url.parse::<hyper::Uri>()?;

        let host = url.host().expect("URI has no host");
        let port = url.port_u16().unwrap_or(80);

        let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let authority = url.authority().unwrap().clone();

        let path = url.path();
        let req = Request::builder()
            .uri(path)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;

        let mut res = sender.send_request(req).await?;

        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        while let Some(next) = res.frame().await {
            let frame = next?;
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk).await?;
            }
        }

        Ok(())
    }

//    pub fn init(cfg: Config) {
//
//        let mut guard = LICENSES.lock().unwrap();
//
//        if let Some(lic) = guard.as_mut() {
//            for l in lic.iter_mut() {
//                l.file_path = cfg.data_dir();
//                let uri = l.remote_url.parse()?;
//                let f = cfg.data_dir().push(l.name);
//            }
//        }
//    }
}


pub fn get_cmd() -> clap::Command {
    clap::Command::new("license")
        .about("license")
        .arg(
            Arg::new("LICENSE")
            .required(true),
        )
        .arg_required_else_help(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Command, Arg};

    fn mock_cli() -> Command {
        Command::new("addlicense")
            .subcommand(
                get_cmd()
            )
    }

    #[test]
    fn valid_license() {
        let matches = mock_cli().get_matches_from(vec![
            "addlicense", "license", "MIT"
        ]);

        if let Some(license_matches) = matches.subcommand_matches("license") {
            let license_value = license_matches.get_one::<String>("name").unwrap();
            assert!(LICENSES.clone().unwrap().iter().any(|item| item.name == *license_value));
        }
    }

    #[test]
    fn invalid_license() {
        let matches = mock_cli().get_matches_from(vec![
            "addlicense", "license", "NotALicense"
        ]);

        if let Some(license_matches) = matches.subcommand_matches("license") {
            let license_value = license_matches.get_one::<String>("name").unwrap();
            assert!(!LICENSES.clone().unwrap().iter().any(|item| item.name == *license_value));
        }
    }
}

