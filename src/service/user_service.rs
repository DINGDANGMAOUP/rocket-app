use crate::common::pojo::dto::request::query::UserPageQuery;
use crate::common::pojo::dto::request::user_request::UserCreateRequest;
use crate::common::utils::password_util::encode;
use crate::domain::table::user::User;
use crate::error::Error;
use crate::mapper::user_mapper;
use actix_web::web::Data;
use rbatis::plugin::page::PageRequest;
use rbatis::{Page, RBatis};

pub async fn create(rb: &Data<RBatis>, data: &UserCreateRequest) {
    let user = User {
        common: Default::default(),
        username: data.username.clone(),
        password: Some(encode(data.password.clone().unwrap())),
        nick_name: data.nick_name.clone(),
        phone: data.phone.clone(),
        email: data.email.clone(),
        sex: data.sex,
        enable: Some(true),
    };
    User::insert(&***rb, &user).await.unwrap();
}
pub async fn page_list(rb: &Data<RBatis>, params: &UserPageQuery) -> Result<Page<User>, Error> {
    let page = user_mapper::select_page_by_params(
        &***rb,
        &PageRequest::new(params.common.page_no, params.common.page_size),
        params,
    )
    .await?;
    Ok(page)
}
