use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::table::common_table::CommonTable;

//用户表
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(flatten)]
    pub common: CommonTable,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sex: Option<i32>,
    pub enable: Option<bool>,
}
crud!(User {}, "t_user");

