use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Clone, Debug, Serialize, Deserialize,Validate)]
pub struct User {
    pub id: Option<i32>,
    #[validate(length(min = 1, max = 20))]
    pub username: Option<String>,
    pub password: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub nick_bame: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub sex: Option<i32>,
}
crud!(User {}, "t_user");
