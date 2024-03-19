use actix::{Actor, StreamHandler};
use actix_web::{Error, get, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::trace;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

struct WebSocket;
impl WebSocket {
    pub fn new() -> Self {
        WebSocket {}
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ProtocolError>> for WebSocket {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        trace!("Handling web-socket item: {:?}", item);
        match item {
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Text(text)) => ctx.text(text),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("")]
pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let ws_actor = WebSocket::new();
    let response = ws::start(ws_actor, &req, stream);

    trace!("Returning {:?}", response);

    response
}
