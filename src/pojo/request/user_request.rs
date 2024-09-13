use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone,Validate)]
pub struct UserCreateRequest {
    #[validate(length(min = 1, max = 20))]
    pub username: Option<String>,
    #[validate(length(min = 6))]
    pub password: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub sex: Option<i32>,
}