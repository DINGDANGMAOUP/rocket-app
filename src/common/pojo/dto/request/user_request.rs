use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Validate)]
pub struct UserCreateRequest {
    #[validate(length(min = 1, max = 20, message = "username is invalid"))]
    pub username: Option<String>,
    #[validate(length(min = 6, message = "password is required"))]
    pub password: Option<String>,
    #[validate(length(min = 1, max = 20, message = "nick_name is invalid"))]
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    #[validate(email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(range(min = 0, max = 10, message = "sex is invalid"))]
    pub sex: Option<i32>,
}
