extern crate actix;
extern crate actix_web;
extern crate diesel;

use self::actix::prelude::*;
use self::actix_web::{error, http};
use self::diesel::prelude::*;
use self::diesel::dsl::{exists, select};
use self::diesel::pg::PgConnection;
use super::db_messages::CreateBackground;
use super::models;

pub struct DbExec(pub PgConnection);

impl Actor for DbExec {
    type Context = SyncContext<Self>;
}

impl Handler<CreateBackground> for DbExec {
    type Result = Result<Vec<models::Gradient>, error::Error>;

    fn handle(&mut self, msg: CreateBackground, _: &mut Self::Context) -> Self::Result
    {
        use lib::schema::messages;
        use lib::schema::colors::dsl::*;
        use lib::schema::gradients;

        let new_message = models::NewMessage {
            message: &msg.message,
        };

        let m = match diesel::insert_into(messages::table)
            .values(&new_message)
            .get_result::<models::Message>(&self.0) {
                Ok(result) => result,
                Err(_) => return Err(error::Error::from(
                    error::InternalError::new(
                        format!("Failed to create message: {}", new_message.message),
                        http::StatusCode::INTERNAL_SERVER_ERROR
                    )
                ))
            };

        let mut cs: Vec<models::Color> = Vec::new();
        for hex in msg.hexcodes {
            match select(exists(colors.filter(hexcode.eq(&hex)))).get_result::<bool>(&self.0) {
                Ok(true) => {
                    match colors.filter(hexcode.eq(&hex)).load::<models::Color>(&self.0) {
                        Ok(mut results) => cs.push(results.pop().unwrap()),
                        Err(_) => return Err(error::Error::from(
                            error::InternalError::new(
                                format!("Failed to retrieve color {}", hex),
                                http::StatusCode::INTERNAL_SERVER_ERROR
                            )
                        ))
                    };
                }, Ok(false) => {
                    let new_color = models::NewColor { hexcode: &hex };
                    let color = match diesel::insert_into(colors)
                        .values(&new_color)
                        .get_result::<models::Color>(&self.0) {
                            Ok(result) => result,
                            Err(_) => return Err(error::Error::from(
                                error::InternalError::new(
                                    format!("Failed to create color: {}", new_color.hexcode),
                                    http::StatusCode::INTERNAL_SERVER_ERROR
                                )
                            ))
                        };
                    cs.push(color);
                }, Err(_) => return Err(error::Error::from(
                    error::InternalError::new(
                        "Exists query failed",
                        http::StatusCode::INTERNAL_SERVER_ERROR
                    )
                ))
            };
        }

        let mut gs: Vec<models::Gradient> = Vec::new();
        for (i, color) in cs.iter().enumerate() {
            let new_gradient = models::NewGradient{
                message_id: m.id,
                color_id: color.id,
                position: i as i32
            };
            let g = match diesel::insert_into(gradients::table)
                .values(&new_gradient)
                .get_result::<models::Gradient>(&self.0) {
                    Ok(result) => result,
                    Err(_) => return Err(error::Error::from(
                        error::InternalError::new(
                            format!("Failed to create gradient: message_id: {}, color_id: {}", new_gradient.message_id, new_gradient.color_id),
                            http::StatusCode::INTERNAL_SERVER_ERROR
                        )
                    ))
                };
            gs.push(g);
        }

        Ok(gs)
    }
}
