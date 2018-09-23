use super::models::{Speech, SpeechData};
use actix::Message;
use actix_web::error::Error;

#[derive(Deserialize)]
pub struct CreateSpeech {
    pub message: String,
    pub hexcodes: Vec<String>,
}

impl Message for CreateSpeech {
    type Result = Result<Speech, Error>;
}

#[derive(Deserialize)]
pub struct GetSpeeches {
    pub page: i32,
}

impl Message for GetSpeeches {
    type Result = Result<SpeechData, Error>;
}
