#[macro_use]
extern crate rbatis;
use actix_web::{
     middleware::Logger, web, App, HttpServer
};
use rbatis::{dark_std::defer, RBatis};
mod config;
mod controller;
mod domain;
mod mapper;
mod pojo;
mod service;

//监听systemconfig 发生变化时重新加载配置rbatis


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
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(rb.clone()))
            .wrap(Logger::default())
            .service(controller::test_controller::hello)
            .service(controller::test_controller::echo)
            .route("/index", web::post().to(controller::test_controller::index))
            .route("/hey", web::get().to(controller::test_controller::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
