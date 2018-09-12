use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use std::env;

pub mod db_exec;
pub mod db_messages;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
