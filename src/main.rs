extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate listenfd;
extern crate openssl;

use actix_web::server;
use dotenv::dotenv;
use listenfd::ListenFd;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

mod lib;

fn main() {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| lib::app::setup_app());

    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| String::from("development"));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else if rust_env == "production" {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();

        server.bind_ssl("127.0.0.1:443", builder).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}
