extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate futures;

use self::actix::{Addr, SyncArbiter};
use self::futures::Future;
use super::orm::db_messages::CreateThing;
use actix_web::{
    http, middleware, ws, App, AsyncResponder, FutureResponse, HttpResponse, Json, State,
};
use dotenv::dotenv;
use std::env;

use super::orm;

mod socket;

pub struct AppState {
    db: Addr<orm::db_exec::DbExec>,
}

fn create_thing(
    (state, json): (State<AppState>, Json<CreateThing>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(json.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(gradients) => Ok(HttpResponse::Ok().json(gradients)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn setup_app() -> App<AppState> {
    dotenv().ok();

    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| String::from("development"));

    let addr = SyncArbiter::start(3, || orm::db_exec::DbExec(orm::establish_connection()));

    App::with_state(AppState { db: addr.clone() })
        .configure(|app| {
            let origin = if rust_env == "production" {
                "https://www.thingspeoplesay.net"
            } else {
                "http://localhost:8080"
            };

            middleware::cors::Cors::for_app(app)
                .allowed_origin(origin)
                .resource("/things", |r| {
                    r.method(http::Method::GET).f(|_| HttpResponse::Ok());
                    r.method(http::Method::POST).with(create_thing);
                })
                .register()
        })
        .middleware(middleware::Logger::default())
        .middleware(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))
        .resource("/color", |r| r.f(|req| ws::start(req, socket::Socket)))
}
