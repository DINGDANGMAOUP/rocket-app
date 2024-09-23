use crate::common::pojo::dto::request::auth_request::LoginRequest;
use actix_web::web;
use rbatis::RBatis;

pub async fn login(rb: web::Data<RBatis>, params: web::Json<LoginRequest>) {}
