extern crate actix;
extern crate actix_web;

use self::actix::{Actor, StreamHandler};
use self::actix_web::ws;
use super::State;

pub struct Socket;

impl Actor for Socket {
    type Context = ws::WebsocketContext<Self, State>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Socket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}
