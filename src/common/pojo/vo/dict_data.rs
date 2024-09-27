use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct DictVo {
    pub dict_name: Option<String>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
}
