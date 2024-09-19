use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use validator::Validate;

pub const ASC: &str = "ASC";
pub const DESC: &str = "DESC";
const DEFAULT_PAGE_SIZE: u64 = 10;

#[derive(Debug, Deserialize, Serialize, Validate, Clone, PartialEq)]
// #[serde_as]
#[serde(rename_all = "camelCase")]
pub struct PageQuery {
    // #[validate(range(min = 1, max = 100, message = "page_size is valid!"))]
    #[serde(rename = "pageSize", skip_serializing_if = "is_default")]
    pub page_size: String,
    // #[validate(range(min = 0, message = "page_size is invalid!"))]
    #[serde(rename = "pageIndex", skip_serializing_if = "is_default")]
    // #[serde_as(as = "DisplayFromStr")]
    pub page_index: String,
    #[serde(rename = "orderBy", skip_serializing_if = "is_default")]
    pub order_by: Option<String>,
    #[serde(rename = "orderDirection", skip_serializing_if = "is_default")]
    pub order_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
}
impl Default for PageQuery {
    fn default() -> Self {
        PageQuery {
            page_size: "10".to_string(),
            page_index: "0".to_string(),
            order_by: None,
            order_direction: Some(DESC.to_string()),
            group_by: None,
        }
    }
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
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
