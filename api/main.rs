use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use std::fs;

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let html = fs::read_to_string("./api/index.html").expect("Something went wrong reading the file");
    Ok(Response::new(Body::from(html)))
}

async fn run_server() {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(serve_req)) }
    });

    let addr = ([127, 0, 0, 1], 6969).into();

    let server = hyper::server::Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    run_server().await;
}
