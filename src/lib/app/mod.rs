extern crate actix;
extern crate actix_web;
extern crate dotenv;

use self::actix::{Addr, SyncArbiter};
use actix_web::{App, http, HttpResponse, middleware, ws};
use dotenv::dotenv;
use std::env;

use super::orm;

mod socket;

pub struct State {
    db: Addr<orm::db_exec::DbExec>
}

pub fn setup_app() -> App<State> {
    dotenv().ok();

    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| String::from("development"));

    let addr = SyncArbiter::start(3, || {
        orm::db_exec::DbExec(orm::establish_connection())
    });

    App::with_state(State{db: addr.clone()})
        .configure(|app| {
            let origin = if rust_env == "production" {
                "https://www.thingspeoplesay.net"
            } else {
                "http://localhost:8080"
            };

            middleware::cors::Cors::for_app(app)
                .allowed_origin(origin)
                .resource("/things", |r| { r.method(http::Method::GET).f(|_| HttpResponse::Ok()) })
                .resource("/things", |r| { r.method(http::Method::POST).f(|_| HttpResponse::Ok()) })
                .register()
        })
        .middleware(middleware::Logger::default())
        .middleware(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))
        .resource("/color", |r| r.f(|req| ws::start(req, socket::Socket)))
}
