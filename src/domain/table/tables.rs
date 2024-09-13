use rbatis::rbdc::DateTime;
use rbatis::snowflake::new_snowflake_id;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::enums::menu_type::MenuType;

/// 公共字段
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommonTable {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub del_flag: Option<i32>,
}
impl Default for CommonTable{
    fn default() -> Self {
       Self{
           id: Some(new_snowflake_id() as i32),
           create_time: Some(DateTime::now()) ,
           update_time: Some(DateTime::now()),
           create_by: None,
           update_by: None,
           remark: None,
           del_flag: Some(0),
       }
    }
}
/// 用户表
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(flatten)]
    pub common: CommonTable,
    #[validate(length(min = 1, max = 20))]
    pub username: Option<String>,
    pub password: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub sex: Option<i32>,
}
crud!(User {}, "t_user");

/// 角色表
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Role {
    #[serde(flatten)]
    pub common: CommonTable,
    pub role_name: Option<String>,
    pub authority: Option<String>,
}

crud!(Role {}, "t_role");

/// 用户角色表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(flatten)]
    pub common: CommonTable,
    pub user_id: i32,
    pub role_id: i32,
}

crud!(UserRole {}, "t_user_role");

/// 菜单表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
    #[serde(flatten)]
    pub common: CommonTable,
    pub menu_name: Option<String>,
    pub parent_id: Option<i32>,
    pub order_num: Option<i32>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub is_frame: Option<i32>,
    pub is_cache: Option<i32>,
    pub is_show: Option<i32>,
    pub status: Option<i32>,
    pub perms: Option<String>,
    pub icon: Option<String>,
    pub type_: Option<MenuType>,
}

crud!(Menu {}, "t_menu");

/// 角色菜单表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleMenu {
    #[serde(flatten)]
    pub common: CommonTable,
    pub role_id: i32,
    pub menu_id: i32,
}

crud!(RoleMenu {}, "t_role_menu");
