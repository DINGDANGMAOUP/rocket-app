use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nick_bame: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sex: Option<i32>,
}
crud!(User {}, "t_user");
