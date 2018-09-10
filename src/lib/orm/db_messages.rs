extern crate actix;
extern crate actix_web;

use self::actix::Message;
use self::actix_web::error::Error;
use super::models::Speech;

#[derive(Deserialize, Serialize)]
pub struct CreateSpeech {
    pub message: String,
    pub hexcodes: Vec<String>,
}

impl Message for CreateSpeech {
    type Result = Result<Speech, Error>;
}

pub struct GetSpeeches {}

impl Message for GetSpeeches {
    type Result = Result<Vec<Speech>, Error>;
}
