extern crate actix;
extern crate actix_web;

use self::actix::Message;
use self::actix_web::error::Error;
use super::models::Gradient;

pub struct CreateBackground {
    pub message: String,
    pub hexcodes: Vec<String>
}

impl Message for CreateBackground {
    type Result = Result<Vec<Gradient>, Error>;
}
