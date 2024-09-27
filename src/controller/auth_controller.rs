use crate::common::pojo::dto::request::auth_request::{LoginRequest, RegisterRequest};
use crate::error::Error;
use crate::response::Response;
use crate::service::auth_service;
use actix_web::{web, HttpResponse};
use rbatis::RBatis;

pub async fn login(
    rb: web::Data<RBatis>,
    params: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let data = auth_service::authenticate(&rb, &*params).await?;
    Ok(Response::build_data(&data))
}

pub async fn register(
    rb: web::Data<RBatis>,
    params: web::Json<RegisterRequest>,
) -> Result<HttpResponse, Error> {
    auth_service::register(&rb, &*params).await?;
    Ok(Response::build_success())
}
