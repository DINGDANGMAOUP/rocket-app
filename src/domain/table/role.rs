use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::table::common_table::CommonTable;

/// 角色表
#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct Role {
    #[serde(flatten)]
    pub common: CommonTable,
    pub role_name: Option<String>,
    pub authority: Option<String>,
}

crud!(Role {}, "t_role");