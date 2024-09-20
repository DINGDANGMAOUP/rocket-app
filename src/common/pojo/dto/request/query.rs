use serde::de::{self};
use serde::{Deserialize, Deserializer, Serialize};
use validator::Validate;
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageQuery {
    #[validate(range(min = 1, max = 100, message = "pageSize is invalid"))]
    #[serde(deserialize_with = "deserialize_json_string")]
    pub page_size: u64,
    #[validate(range(min = 1, max = 9999, message = "pageNo is invalid"))]
    #[serde(deserialize_with = "deserialize_json_string")]
    pub page_no: u64,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
    pub group_by: Option<String>,
}

fn deserialize_json_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    serde_json::from_str(s).map_err(de::Error::custom)
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
