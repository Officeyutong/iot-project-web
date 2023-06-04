use actix_web::{get, web, Responder, Scope};
use serde::{Deserialize, Serialize};

pub mod get_data;
pub mod websocket;
pub fn make_scope() -> Scope {
    return web::scope("/api")
        .service(echo)
        .service(websocket::handle_ws)
        .service(get_data::get_data);
}

#[derive(Deserialize)]
struct EchoReq {
    text: String,
}

#[derive(Serialize)]
struct EchoResp {
    text: String,
}

#[get("echo")]
async fn echo(params: web::Query<EchoReq>) -> impl Responder {
    web::Json(EchoResp {
        text: params.text.clone(),
    })
}
