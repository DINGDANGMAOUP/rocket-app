use serde::{Deserialize, Serialize};
use crate::domain::table::common_table::CommonTable;

/// 用户角色表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(flatten)]
    pub common: CommonTable,
    pub user_id: i32,
    pub role_id: i32,
}

crud!(UserRole {}, "t_user_role");
