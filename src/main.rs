use futures::future::{self, Future};
use hyper::server::{Http, Request, Response, Service};
use hyper::Error;
use log::info;

struct Microservice;

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        info!("Microservice received a request: {:?}", request);
        Box::new(future::ok(Response::new()))
    }
}

fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new()
        .bind(&address, || Ok(Microservice {}))
        .unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}