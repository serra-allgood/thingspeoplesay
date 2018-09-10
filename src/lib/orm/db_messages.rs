extern crate actix;
extern crate actix_web;

use self::actix::Message;
use self::actix_web::error::Error;
use super::models::Gradient;

#[derive(Deserialize)]
pub struct CreateThing {
    pub message: String,
    pub hexcodes: Vec<String>,
}

impl Message for CreateThing {
    type Result = Result<Vec<Gradient>, Error>;
}
