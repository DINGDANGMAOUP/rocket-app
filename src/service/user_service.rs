use crate::domain::table::tables::User;
use crate::pojo::request::user_request::UserCreateRequest;
use actix_web::web::Data;
use rbatis::RBatis;

pub async fn create(rb: Data<RBatis>, data: &UserCreateRequest) {
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
    User::insert(&**rb, &user).await.unwrap();
}
