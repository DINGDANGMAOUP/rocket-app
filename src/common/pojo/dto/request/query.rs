use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageQuery {
    pub page_size: String,
    pub page_no: String,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
    pub group_by: Option<String>,
}


#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPageQuery {
    #[serde(flatten)]
    pub common: PageQuery,
    pub id: Option<i64>,
    pub username: Option<String>,
    pub nick_name: Option<String>,
    pub enable: Option<bool>,
}
