extern crate diesel;

use super::db_messages::*;
use super::models;
use actix::prelude::*;
use actix_web::error;
use diesel::{
    dsl::{exists, select, sql_query},
    pg::PgConnection,
    prelude::*,
};

pub struct DbExec(pub PgConnection);

impl Actor for DbExec {
    type Context = SyncContext<Self>;
}

impl Handler<CreateSpeech> for DbExec {
    type Result = Result<models::Speech, error::Error>;

    fn handle(&mut self, msg: CreateSpeech, _: &mut Self::Context) -> Self::Result {
        use lib::schema::colors::dsl::*;
        use lib::schema::gradients;
        use lib::schema::messages;

        let new_message = models::NewMessage {
            message: msg.message.as_str(),
        };

        let m = match diesel::insert_into(messages::table)
            .values(&new_message)
            .get_result::<models::Message>(&self.0)
        {
            Ok(result) => result,
            Err(_) => {
                return Err(error::ErrorInternalServerError(format!(
                    "Failed to create message: {}",
                    new_message.message
                )))
            }
        };

        let mut cs: Vec<models::Color> = Vec::new();
        for hex in msg.hexcodes.iter() {
            match select(exists(colors.filter(hexcode.eq(&hex)))).get_result::<bool>(&self.0) {
                Ok(true) => {
                    match colors
                        .filter(hexcode.eq(&hex))
                        .load::<models::Color>(&self.0)
                    {
                        Ok(mut results) => cs.push(results.pop().unwrap()),
                        Err(_) => {
                            return Err(error::ErrorInternalServerError(format!(
                                "Failed to retrieve color {}",
                                hex
                            )))
                        }
                    };
                }
                Ok(false) => {
                    let new_color = models::NewColor { hexcode: &hex };
                    let color = match diesel::insert_into(colors)
                        .values(&new_color)
                        .get_result::<models::Color>(&self.0)
                    {
                        Ok(result) => result,
                        Err(_) => {
                            return Err(error::ErrorInternalServerError(format!(
                                "Failed to create color: {}",
                                new_color.hexcode
                            )))
                        }
                    };
                    cs.push(color);
                }
                Err(_) => return Err(error::ErrorInternalServerError("Exists query failed")),
            };
        }

        for (i, color) in cs.iter().enumerate() {
            let new_gradient = models::NewGradient {
                message_id: m.id,
                color_id: color.id,
                position: i as i32,
            };
            match diesel::insert_into(gradients::table)
                .values(&new_gradient)
                .execute(&self.0)
            {
                Ok(_) => (),
                Err(_) => {
                    return Err(error::ErrorInternalServerError(format!(
                        "Failed to create gradient: message_id: {}, color_id: {}",
                        new_gradient.message_id, new_gradient.color_id
                    )))
                }
            };
        }

        let thing = models::Speech {
            message: msg.message.clone(),
            hexcodes: msg.hexcodes,
        };

        Ok(thing)
    }
}

impl Handler<GetSpeeches> for DbExec {
    type Result = Result<models::SpeechData, error::Error>;

    fn handle(&mut self, msg: GetSpeeches, _: &mut Self::Context) -> Self::Result {
        match sql_query(format!(
            "SELECT max(num) AS total, array_agg(jsonb_build_object('message', message, 'hexcodes', hexcodes)) AS speeches
            FROM (
                SELECT
                    messages.message,
                    array_agg(colors.hexcode) AS hexcodes,
                    row_number() OVER (ORDER BY messages.created_at DESC) AS num
                FROM messages
                INNER JOIN gradients ON gradients.message_id = messages.id
                INNER JOIN colors ON colors.id = gradients.color_id
                GROUP BY messages.id
            ) AS messages
            WHERE num BETWEEN {} AND {}",
            (msg.page - 1) * 200,
            (msg.page - 1) * 200 + 200
        )).get_result::<models::SpeechData>(&self.0)
        {
            Ok(results) => Ok(results),
            Err(_) => Err(error::ErrorInternalServerError("Failed to fetch speeches")),
        }
    }
}
