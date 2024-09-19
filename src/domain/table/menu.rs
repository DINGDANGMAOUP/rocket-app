use serde::{Deserialize, Serialize};
use crate::common::constants::menu_type::MenuType;
use crate::domain::table::common_table::CommonTable;

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