use http_body_util::{Empty, BodyExt};
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt as _};
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;

pub trait SdkClient {
    fn do(&self, url: &str) -> Result<(), String>;
    fn get(&self, url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct HttpClient;

impl HttpClient {

    pub fn new() -> Self {
        HttpClient
    }
}

impl SdkClient for HttpClient {

    // TODO: common entrypoint `do` with param `url` and `op` enum (get,post,...) in which
    // all pre-parsing and flight-checks are done
    //

    async fn do(&self, url: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create HTTP client runtime: {}", e))?;

        runtime.block_on(async {
            get(url)
        })
    }

    async fn get(&self, url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        let uri = url.parse::<hyper::Uri>()?; //{
        //    Ok(u) => u,
        //    Err(e) => {
        //        return Err(format!("Could not parse URL '{}': {}", url, e));
        //    }
        //};
        //
        let host = uri.host().expect("URI has no host");
        let port = uri.port_u16().unwrap_or(80);

        let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;// {
        //    Ok(s) => s,
        //    Err(e) => {
        //        return Err(format!("Could not connect to host '{}': {}", host, e));
        //    }
        //};
        //
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?; // {
        //    Ok((s, c)) => (s, c),
        //    Err(e) => {
        //        return Err(format!("Could not establish handshake: {}", e));
        //    }
        //};
        //
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
            .body(Empty::<Bytes>::new())?;// {
            //Ok(r) => r,
            //Err(e) => {
            //    return Err(format!("Could not send request: {}", e));
            //}

        //};

        let mut res =sender.send_request(req).await?;// {
        //    Ok(r) => r,
        //    Err(e) => {
        //        return Err(format!("Error occurred while receiving response: {}", e));
        //    }
        //};
        //
        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        while let Some(next) = res.frame().await {
            let frame = next?;// {
            //    Ok(f) => f,
            //    Err(e) => {
            //        return Err(format!("{}", e));
            //    }
            //};
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk).await?;// {
            //        Ok(c) => c,
            //        Err(e) => {
            //            return Err(format!("{}", e));
            //        }
            //    };
            }
        }

        Ok(())
    }

}


