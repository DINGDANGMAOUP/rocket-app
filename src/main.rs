use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use rust_platform::middleware::filter::jwt_filter::JWTFilter;
use rust_platform::middleware::filter::test_filter::SayHi;
use rust_platform::{
    config,
    controller::{index_controller, user_controller},
    domain,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::config::SystemConfig::default();
    config::log::init_log(&config);
    let rb = config::db::init_db(&config).await;
    domain::table::table_init::sync_tables(&rb).await;
    domain::table::table_init::sync_tables_data(&rb).await;
    let url = &config.server.host;
    let port = &config.server.port;
    let server_url = format!("{}:{}", url, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.to_owned()))
            .app_data(web::Data::new(rb.to_owned()))
            .wrap(JWTFilter)
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(
                web::scope("/ui")
                    .route("", web::get().to(index_controller::index))
                    .route("/{_:.*}", web::get().to(index_controller::dist)),
            )
            .service(
                web::scope("/user")
                    .route("", web::post().to(user_controller::create))
                    .route("", web::get().to(user_controller::list)),
            )
        // .route("/index", web::post().to(index))
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(&server_url)?
    .run()
    .await
}
