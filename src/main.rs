extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix::System;
use actix_web::server;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

mod lib;

fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = System::new("thingspeoplesay");

    let mut server = server::new(|| lib::app::setup_app());

    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| String::from("development"));

    server = if rust_env == "production" {
        server.bind("0.0.0.0:80").unwrap()
    } else {
        server.bind("localhost:8080").unwrap()
    };

    server.start();
    sys.run();
}
