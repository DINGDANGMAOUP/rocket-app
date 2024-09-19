use crate::domain::table::common_table::CommonTable;
use serde::{Deserialize, Serialize};

/// 用户角色表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(flatten)]
    pub common: CommonTable,
    pub user_id: i32,
    pub role_id: i32,
}

crud!(UserRole {}, "t_user_role");
