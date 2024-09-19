use crate::domain::table::user::User;
use crate::common::pojo::dto::request::query::UserPageQuery;

htmlsql_select_page!(select_page_by_params(params:&UserPageQuery)->User=>"sql/html/user.html");