use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::constants::menu_type::MenuType;
use crate::pojo::dto::query::UserPageQuery;

/// 公共字段
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommonTable {
    pub id: Option<i64>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub del_flag: Option<i32>,
}
impl Default for CommonTable {
    fn default() -> Self {
        Self {
            id: None,
            create_time: Some(DateTime::now()),
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
    pub username: Option<String>,
    pub password: Option<String>,
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sex: Option<i32>,
    pub enable: Option<bool>,
}
crud!(User {}, "t_user");
/*
impl_select_page!(BizActivity{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});
 */
impl_select_page!(User{select_page_by_params(params:&UserPageQuery)=>r#"
if params != null && params.id != null && params.id != '':
    `where id = #{params.id}`
if params != null && params.username != null && params.username != '' :
    `where username ~* #{params.username}`
if params != null && params.nick_name != null && params.nick_name != '':    
    `where nick_name ~* #{params.nick_name}`
if params != null && params.enable !=null && params.enable != '':
    `where nick_name = #{params.enable}`   
       
"#},"t_user");

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
///字典表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictType {
    #[serde(flatten)]
    pub common: CommonTable,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i32>,
}
crud!(DictType {}, "t_dict_type");
///字典详情表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictData {
    #[serde(flatten)]
    pub common: CommonTable,
    pub dict_type_id: Option<i32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_sort: Option<i32>,
}
crud!(DictData {}, "t_dict_data");
