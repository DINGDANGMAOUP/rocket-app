#[macro_use]
extern crate diesel;

use actix_web::{
    error, get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use diesel::{prelude::*, r2d2};
mod actions;
mod config;
mod dto;
mod models;
mod schema;
type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[get("/list")]
async fn list(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let us = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::all(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(us))
}
/// extract `Info` using serde
async fn index(info: web::Json<dto::Info>) -> Result<String> {
    log::debug!("model::person::Info: {:?}", info);
    Ok(format!("Welcome {}!", info.username))
}
async fn manual_hello() -> impl Responder {
    let res = dto::User {
        name: "John".to_string(),
        age: 32,
    };
    HttpResponse::Ok().json(res)
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost:5432/postgres",
    );
    env_logger::init();
    let pool = initialize_db_pool();
    let config = config::init_config();
    // env_logger::init_from_env(Env::default().default_filter_or("deubg"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .service(list)
            .route("/index", web::post().to(index))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
