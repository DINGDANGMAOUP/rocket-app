use crate::response::ResponseErr;
use actix_web::HttpResponse;
use mime_guess::from_path;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "resource/"]
struct Asset;

pub fn handle_web_file(path: &str) -> HttpResponse {
    match Asset::get(format!("ui/{}", path).as_ref()) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().json(ResponseErr {
            success: false,
            err_code: "404".to_string(),
            err_message: Some("Not Found".to_string()),
        }),
    }
}
