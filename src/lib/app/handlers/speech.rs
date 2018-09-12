extern crate futures;

use self::futures::Future;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, Query, State};
use lib::{app, orm::db_messages::*};

pub fn create_speech(
    (state, json): (State<app::AppState>, Json<CreateSpeech>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(json.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(speech) => Ok(HttpResponse::Ok().json(speech)),
            Err(err) => Ok(HttpResponse::from_error(err)),
        })
        .responder()
}

pub fn get_speeches(
    (state, query): (State<app::AppState>, Query<GetSpeeches>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(query.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(speeches) => Ok(HttpResponse::Ok().json(speeches)),
            Err(err) => Ok(HttpResponse::from_error(err)),
        })
        .responder()
}
