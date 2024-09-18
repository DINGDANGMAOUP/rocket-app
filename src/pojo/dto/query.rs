use serde::{Deserialize, Serialize};
use validator::Validate;

pub const ASC: &str = "ASC";
pub const DESC: &str = "DESC";
const DEFAULT_PAGE_SIZE: i32 = 10;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PageQuery {
    #[validate(range(min = 1, max = 100, message = "page_size is valid!"))]
    #[serde(skip_serializing_if = "is_default")]
    page_size: i32,
    #[validate(range(min = 0, message = "page_size is invalid!"))]
    #[serde(skip_serializing_if = "is_default")]
    page_index: i32,
    #[serde(rename = "orderBy", skip_serializing_if = "is_default")]
    order_by: String,
    #[serde(rename = "orderDirection", skip_serializing_if = "is_default")]
    order_direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by: Option<String>,
}
impl Default for PageQuery {
    fn default() -> Self {
        PageQuery {
            page_size: DEFAULT_PAGE_SIZE,
            page_index: 0,
            order_by: "id".to_string(),
            order_direction: DESC.to_string(),
            group_by: None,
        }
    }
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserPageQuery {
    #[serde(flatten)]
    pub common: PageQuery,
    pub id: Option<i64>,
    pub username: Option<String>,
    pub nick_name: Option<String>,
    pub enable: Option<bool>,
}
