use crate::domain::table::common_table::CommonTable;
use serde::{Deserialize, Serialize};

///字典详情表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictDetail {
    #[serde(flatten)]
    pub common: CommonTable,
    pub dict_type_id: Option<i32>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub dict_sort: Option<i32>,
}
crud!(DictDetail {}, "t_dict_detail");
