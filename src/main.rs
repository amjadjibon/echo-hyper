use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Request, Response, Server, StatusCode};
use log::{debug, error, info};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        _ => {
            debug!("Path: {}", req.uri());
            debug!("Method: {}", req.method());
            debug!("Headers: {:?}", req.headers());
            debug!("Body: {:?}", hyper::body::to_bytes(req.into_body()).await?);

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap();
            Ok(response)
        }
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn run_server() {
    let port;
    match env::var("PORT") {
        Ok(val) => port = val,
        Err(_e) => port = "8080".to_string(),
    };

    let port = port.parse::<u16>().unwrap();

    env_logger::init();
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    info!("server running on http://{}", addr);
    if let Err(e) = graceful.await {
        error!("server error: {}", e);
    }
    info!(" gracefully shutdown complete")
}

#[tokio::main]
async fn main() {
    run_server().await;
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::{body::to_bytes, Client, Method};
    use tokio::runtime::Runtime;
    #[test]
    fn test_echo() {
        let rt = Runtime::new().unwrap();

        // start server
        rt.spawn(run_server());

        // wait for server to come up
        std::thread::sleep(std::time::Duration::from_millis(50));

        let client = Client::new();

        // make requests
        let req = client.request(
            Request::builder()
                .method(Method::GET)
                .uri("http://localhost:8080/echo")
                .body(Body::empty())
                .unwrap(),
        );

        let res = rt.block_on(req).unwrap();
        let body = rt.block_on(to_bytes(res.into_body())).unwrap();

        println!("{:?}", body);

        assert_eq!(std::str::from_utf8(&body).unwrap(), "{}");
    }
}
