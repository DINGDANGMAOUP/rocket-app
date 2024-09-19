use crate::domain::table::user::{select_page_by_params, User};
use crate::error::Error;
use crate::pojo::dto::query::UserPageQuery;
use crate::pojo::request::user_request::UserCreateRequest;
use actix_web::web::Data;
use rbatis::plugin::page::PageRequest;
use rbatis::{Page, RBatis};

pub async fn create(rb: &Data<RBatis>, data: &UserCreateRequest) {
    let user = User {
        common: Default::default(),
        username: data.username.clone(),
        password: data.password.clone(),
        nick_name: data.nick_name.clone(),
        phone: data.phone.clone(),
        email: data.email.clone(),
        sex: data.sex.clone(),
        enable: Some(true),
    };
    User::insert(&***rb, &user).await.unwrap();
}
pub async fn pageList(rb: &Data<RBatis>, params: &UserPageQuery) -> Result<Page<User>, Error> {
    let page = select_page_by_params(
        &***rb,
        &PageRequest::new(
            params.common.page_no.parse::<u64>().unwrap(),
            params.common.page_size.parse::<u64>().unwrap(),
        ),
        params,
    )
    .await?;
    Ok(page)
}
