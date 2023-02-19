
extern crate hyper;
extern crate futures;

#[macro_use]
extern crate log;
extern crate env_logger;


use hyper::server::{Request,Response,Service};
use futures::future::Future;

struct Microservice;

impl service for Microservice{
    type Request =  Request;
    type Response =  Response;
    type Error  =  hype::Error;
    type Future =  Box<Future<Item = Self::Response, Error =  self::Error>>;

    fn call(&self, request::Request) -> Self::Future{
        info!("Microservice Recieved a request:{;?}", request);
        Box::new(futures::future::ok(Reponse::New()))
    }

}

fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(Microservice {}))
        .unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}