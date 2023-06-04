pub mod api;

use crate::State;
use actix_files::NamedFile;
use actix_web::{get, http::header, HttpResponse, Responder};

#[get("/")]
pub async fn index(state: State) -> impl Responder {
    NamedFile::open_async(state.web_dir.join("index.html")).await
}

#[get("/compass.png")]
pub async fn compass_png() -> impl Responder {
    let img = include_bytes!("../compass.png");
    HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "image/png"))
        .body(img.to_vec())
}
