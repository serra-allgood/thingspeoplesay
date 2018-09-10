extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate openssl;
#[macro_use]
extern crate serde_derive;

use actix::System;
use actix_web::server;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

mod lib;

fn main() {
    dotenv().ok();

    let sys = System::new("thingspeoplesay");

    let mut server = server::new(|| lib::app::setup_app());

    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| String::from("development"));

    server = if rust_env == "production" {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();

        server.bind_ssl("localhost:443", builder).unwrap()
    } else {
        server.bind("localhost:3000").unwrap()
    };

    server.start();
    sys.run();
}
