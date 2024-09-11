use serde::{Deserialize, Serialize};
use validator::Validate;



#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct CommonTable {
    pub id: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub del_flag: Option<i32>,
}



#[derive(Clone, Debug, Serialize, Deserialize,Validate)]
pub struct User {
    #[serde(flatten)]
    pub common: CommonTable,
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
