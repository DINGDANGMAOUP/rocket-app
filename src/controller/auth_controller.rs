use crate::common::pojo::dto::request::auth_request::LoginRequest;
use crate::service::user_service;
use actix_web::web;
use rbatis::RBatis;

pub async fn login(rb: web::Data<RBatis>, params: web::Json<LoginRequest>) {
    user_service::authenticate(&rb, &*params).await.unwrap();
}
