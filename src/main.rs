use std::{convert::Infallible, error::Error, net::SocketAddr};

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{
    body::Bytes, server::conn::http1, service::service_fn, Method, Request, Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn get_transactions() -> Result<Response<Full<Bytes>>, Infallible> {
    let response_body = Full::new(Bytes::from("Transactions"));
    Ok(Response::new(response_body))
}

async fn router_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full("Welcome to the Invoice Generating"))),
        (&Method::GET, "/api/transactions") => get_transactions(),

        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
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
                .serve_connection(io, service_fn(router_service))
                .await
            {
                eprintln!(" Error serving connection: {:?}", err);
            }
        });
    }
}
