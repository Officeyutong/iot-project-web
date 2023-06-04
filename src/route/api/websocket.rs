use actix::{Actor, StreamHandler};
use actix_web::{
    get,
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws::{self, CloseCode};
use log::{debug, error, info};
use serde::Deserialize;

use crate::{
    state::{AppState, CompassHeading, GeolocationData, OrientationData, SimpleXYZ},
    State,
};

#[derive(Deserialize, Debug)]
pub enum ClientMessage {
    Accelerometer(SimpleXYZ),
    Gyroscope(SimpleXYZ),
    Orientation(OrientationData),
    Gravity(SimpleXYZ),
    Geolocation(GeolocationData),
    Compass(CompassHeading),
}

struct WebsocketHandler {
    state: Data<AppState>,
}

impl Actor for WebsocketHandler {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let decoded = match serde_json::from_slice::<ClientMessage>(text.as_bytes()) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("Failed to decode what the client said: {},{:?}", e, text);
                        ctx.close(Some((CloseCode::Invalid, "Invalid message").into()));
                        return;
                    }
                };
                debug!("Received: {:#?}", decoded);
                match decoded {
                    ClientMessage::Accelerometer(acc) => {
                        self.state.set_acceleration(acc);
                    }
                    ClientMessage::Gyroscope(gs) => {
                        self.state.add_gyroscope(&gs);
                    }
                    ClientMessage::Orientation(o) => {
                        self.state.set_orientation(o);
                    }
                    ClientMessage::Gravity(g) => self.state.set_gravity(g),
                    ClientMessage::Geolocation(g) => self.state.set_geolocation(g),
                    ClientMessage::Compass(c) => self.state.set_compass(c),
                }
            }
            _ => (),
        }
    }
}

#[get("/upload")]
pub async fn handle_ws(
    req: HttpRequest,
    stream: web::Payload,
    state: State,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebsocketHandler { state }, &req, stream);
    info!("{:?}", resp);
    resp
}
