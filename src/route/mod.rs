pub mod api;

use crate::State;
use actix_files::NamedFile;
use actix_web::{get, Responder};

#[get("/")]
pub async fn index(state: State) -> impl Responder {
    NamedFile::open_async(state.web_dir.join("index.html")).await
}
