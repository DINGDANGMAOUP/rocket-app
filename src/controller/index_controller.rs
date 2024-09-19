use actix_web::{web, Responder};
use crate::common::utils::resource::handle_web_file;

pub async fn index() -> impl Responder {
    handle_web_file("index.html")
}

pub async fn dist(path: web::Path<String>) -> impl Responder {
    handle_web_file(path.as_str())
}