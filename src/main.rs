use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use rust_platform::config::config::SYSTEM_CONFIG;
use rust_platform::controller::{auth_controller, demo_controller};
use rust_platform::security::filter::jwt_filter::JWTFilter;
use rust_platform::{
    config,
    controller::{index_controller, user_controller},
    domain,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = &SYSTEM_CONFIG;
    config::log::init_log(&config);
    let rb = config::db::init_db(&config).await;
    domain::table::table_init::sync_tables(&rb).await;
    domain::table::table_init::sync_tables_data(&rb).await;
    config::dict::init_dict(&rb).await.expect("init dict error");
    let url = &config.server.host;
    let port = &config.server.port;
    let server_url = format!("{}:{}", url, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.to_owned()))
            .app_data(web::Data::new(rb.to_owned()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(JWTFilter)
            .service(
                web::scope("/demo")
                    .route("/auth/login", web::post().to(demo_controller::demo_login))
                    .route("/user/info", web::get().to(demo_controller::demo_user_info))
                    .route("/auth/codes", web::get().to(demo_controller::demo_code)),
            )
            .service(
                web::scope("/ui")
                    .route("", web::get().to(index_controller::index))
                    .route("/{_:.*}", web::get().to(index_controller::dist)),
            )
            .service(web::scope("/oauth").route("/login", web::post().to(auth_controller::login)))
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
