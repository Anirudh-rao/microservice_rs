
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

}



fn call(&self, request::Request) -> Self::Future{
    info!("Microservice Recieved a request:{;?}", request);
    Box::new(futures::future::ok(Reponse::New()))
}
