use crate::error::Error;
use crate::pojo::dto::query::UserPageQuery;
use crate::pojo::request::user_request::UserCreateRequest;
use crate::response::Response;
use crate::service::user_service;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use rbatis::RBatis;
use serde_json::json;
use validator::Validate;

/**
 * 创建用户处理函数。
 * 接收JSON格式的用户数据，创建用户并返回成功响应。
 *
 * # Arguments
 * * `json` - 包含用户信息的JSON对象。
 *
 * # Returns
 * 返回HTTP响应，表示操作成功。
 */
pub async fn create(
    rb: web::Data<RBatis>,
    data: web::Json<UserCreateRequest>,
) -> Result<HttpResponse, Error> {
    Validate::validate(&data.clone())?;
    user_service::create(&rb, &*data).await;
    Ok(Response::build_success())
}
///查询用户列表
pub async fn list(
    rb: web::Data<RBatis>,
    params: web::Query<UserPageQuery>,
) -> Result<HttpResponse, Error> {
    Validate::validate(&*params)?;
    println!("params : {}", json!(&*params));
    let user_page = user_service::pageList(&rb, &*params).await?;
    println!("page :{:?}", &user_page);
    Ok(Response::build_data(&user_page))
}
