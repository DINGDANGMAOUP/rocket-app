use actix_web::{get, post, web, HttpResponse, Responder, Result};
use rbatis::RBatis;
use crate::{domain, pojo};
#[get("/")]
pub  async fn hello(rb: web::Data<RBatis>) -> impl Responder {
    let users = domain::table::tables::User::select_all(&**rb).await.unwrap();
    log::info!("users: {:?}", users);
    HttpResponse::Ok().json(users)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body) 
}

/// extract `Info` using serde
pub async fn index(info: web::Json<pojo::dto::Info>) -> Result<String> {
    log::debug!("model::person::Info: {:?}", info);
    Ok(format!("Welcome {}!", info.username))
}
pub async fn manual_hello() -> impl Responder {
    let res = pojo::dto::User {
        name: "John".to_string(),
        age: 32,
    };
    HttpResponse::Ok().json(res)
}