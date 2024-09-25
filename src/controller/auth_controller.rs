use crate::common::pojo::dto::request::auth_request::LoginRequest;
use crate::error::Error;
use crate::response::Response;
use crate::service::user_service;
use actix_web::{web, HttpResponse};
use rbatis::RBatis;

pub async fn login(
    rb: web::Data<RBatis>,
    params: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let data = user_service::authenticate(&rb, &*params).await?;
    Ok(Response::build_data(&data))
}
