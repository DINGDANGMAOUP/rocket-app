use crate::common::pojo::dto::request::auth_request::{LoginRequest, RegisterRequest};
use crate::common::utils::jwt_util::{gen_jwt, GrantType};
use crate::domain::table::user::User;
use crate::error::Error;
use actix_web::web::Data;
use rbatis::RBatis;
use std::collections::HashMap;

pub async fn authenticate(
    rb: &Data<RBatis>,
    params: &LoginRequest,
) -> Result<HashMap<String, String>, Error> {
    let user = User::select_by_username(&***rb, "id,username,password", &params.username).await?;
    let user = match user {
        Some(v) => v,
        None => return Err(Error::Unauthorized(serde_json::json!("user not found"))),
    };
    if params.password != user.password.unwrap() {
        return Err(Error::Unauthorized(serde_json::json!("password error")));
    }
    let token = gen_jwt(&user.username.unwrap(), GrantType::AccessToken)?;
    let mut map = HashMap::new();
    map.insert(String::from("accessToken"), token);
    Ok(map)
}

pub async fn register(rb: &Data<RBatis>, params: &RegisterRequest) -> Result<(), Error> {
    let user = User {
        common: Default::default(),
        username: Some(params.username.clone()),
        password: Some(params.password.clone()),
        nick_name: Some(params.nick_name.clone()),
        phone: Some(params.phone.clone()),
        email: Some(params.email.clone()),
        sex: None,
        enable: None,
    };
    User::insert(&***rb, &user).await?;
    Ok(())
}
