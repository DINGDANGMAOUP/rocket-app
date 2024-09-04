#[macro_use]
extern crate rbatis;
use actix_web::{middleware::Logger, web, App, HttpServer};
use domain::table::tables::User;
use rbatis::{dark_std::defer, RBatis};
use crate::controller::test_controller::{echo, hello,index,manual_hello};
mod config;
mod controller;
mod domain;
mod mapper;
mod pojo;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });
    let config = config::config::SystemConfig::default();
    let rb = RBatis::new();
    rb.init(rbdc_pg::driver::PgDriver {}, &config.app.datasource.url)
        .unwrap();
    domain::table::table_init::sync_tables(&rb).await;
    domain::table::table_init::sync_tables_data(&rb).await;
    let url = &config.server.host;
    let port = &config.server.port;
    let server_url = format!("{}:{}", url, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(rb.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/index", web::post().to(index))
            .route(
                "/hey",
                web::get().to(manual_hello),
            )
    })
    .bind(&server_url)?
    .run()
    .await
}


pub async fn sync_tables_data(rb: &RBatis) {
    let conn = rb.acquire().await.expect("init data fail");
    if let Ok(v) = User::select_by_column(&conn, "id", "1").await {
        if v.len() > 0 {
            //if user exists,return
            return;
        }
    };
    let users=vec![
        User {
            id: Some(0),
            name: Some("admin".to_string()),
        },
        User {
            id: Some(1),
            name: Some("test".to_string()),
        },
    ];
  let _=  User::insert_batch(&conn, &users, users.len() as u64).await;
  
}