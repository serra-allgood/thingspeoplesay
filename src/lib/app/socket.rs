use super::AppState;
use actix::{Actor, StreamHandler};
use actix_web::ws;
use lib::colors::text_to_colors;

pub struct Socket;

impl Actor for Socket {
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Socket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text_to_colors(&text)),
            _ => (),
        }
    }
}
