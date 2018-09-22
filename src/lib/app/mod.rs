use self::http::header;
use actix::{Addr, SyncArbiter};
use actix_web::{http, middleware, ws, App};
use dotenv::dotenv;
use lib::orm::{db_exec::DbExec, establish_connection};
use std::env;

mod handlers;
mod socket;

pub struct AppState {
    db: Addr<DbExec>,
}

pub fn setup_app() -> App<AppState> {
    dotenv().ok();

    let rust_env = env::var("RUST_ENV").unwrap_or_else(|_| String::from("development"));

    let addr = SyncArbiter::start(3, || DbExec(establish_connection()));

    App::with_state(AppState { db: addr.clone() })
        .configure(|app| {
            let origin = if rust_env == "production" {
                "https://www.thingspeoplesay.net"
            } else {
                "http://localhost:3000"
            };

            middleware::cors::Cors::for_app(app)
                .allowed_origin(origin)
                .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
                .expose_headers(vec![header::ACCESS_CONTROL_ALLOW_ORIGIN])
                .resource("/things", |r| {
                    r.method(http::Method::GET).with(handlers::get_speeches);
                    r.method(http::Method::POST).with(handlers::create_speech);
                })
                .resource("/color", |r| r.f(|req| ws::start(req, socket::Socket)))
                .register()
        })
        .middleware(middleware::Logger::default())
        .middleware(middleware::DefaultHeaders::new().header("Content-Type", "application/json"))
}
