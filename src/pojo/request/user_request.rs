use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone,Validate)]
pub struct UserCreateRequest {
    #[validate(length(min = 1, max = 20,message ="username不符合规范"))]
    pub username: Option<String>,
    #[validate(length(min = 6,message="password不符合规范"))]
    pub password: Option<String>,
    #[validate(length(min = 1, max = 20,message="昵称有误"))]
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    #[validate(email(message="请输入规范的邮箱"))]
    pub email: Option<String>,
    pub sex: Option<i32>,
}