use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
//公共字段
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
