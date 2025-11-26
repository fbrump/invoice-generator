use std::{convert::Infallible, error::Error, net::SocketAddr};

use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let port = 3000;
    let host = [127, 0, 0, 1];
    let address = SocketAddr::from((host, port));
    let listner = TcpListener::bind(address).await?;

    loop {
        let (stream, _) = listner.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!(" Error serving connection: {:?}", err);
            }
        });
    }
}
