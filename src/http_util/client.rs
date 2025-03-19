use std::pin::Pin;
use std::future::Future;
use http_body_util::{Empty, BodyExt};
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt as _};
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;

//pub trait SdkClient {
//    fn exec(&self, url: &str) -> Result<(), String>;
//  fn get(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>>>;
//}

pub struct HttpClient;

//impl HttpClient {
//
//    pub fn new() -> Self {
//        HttpClient
//    }
//}
//
//impl SdkClient for HttpClient {
impl HttpClient {

    pub fn new() -> Self {
        HttpClient
    }

    // TODO: common entrypoint `exec` with param `url` and `op` enum (get,post,...) in which
    // all pre-parsing and flight-checks are done
    //

    pub fn exec(&self, url: String) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create HTTP client runtime: {}", e))?;

        runtime.block_on(async {
            let _ = self.get(url);
            Ok(())
        })
    }

    async fn get(&self, url: String) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>>> {

        Box::pin(async move {
        let uri = url.parse::<hyper::Uri>().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let host = uri.host().expect("URI has no host");
        let port = uri.port_u16().unwrap_or(80);

        let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let authority = uri.authority().unwrap().clone();

        let path = uri.path();
        let req = Request::builder()
            .uri(path)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;

        let mut res =sender.send_request(req).await?;

        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        while let Some(next) = res.frame().await {
            let frame = next?; 
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk).await?;
            }
        }

        Ok(())
        })
    }
}

