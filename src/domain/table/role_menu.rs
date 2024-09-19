use crate::domain::table::common_table::CommonTable;
use serde::{Deserialize, Serialize};

/// 角色菜单表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleMenu {
    #[serde(flatten)]
    pub common: CommonTable,
    pub role_id: i32,
    pub menu_id: i32,
}

crud!(RoleMenu {}, "t_role_menu");
