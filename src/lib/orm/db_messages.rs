use super::models::Speech;
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
    pub start_date: String,
    pub end_date: String,
}

impl Message for GetSpeeches {
    type Result = Result<Vec<Speech>, Error>;
}
