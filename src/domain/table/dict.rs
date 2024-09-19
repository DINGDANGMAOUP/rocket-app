use crate::domain::table::common_table::CommonTable;
use serde::{Deserialize, Serialize};

///字典表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dict {
    #[serde(flatten)]
    pub common: CommonTable,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i32>,
}
crud!(Dict {}, "t_dict");
