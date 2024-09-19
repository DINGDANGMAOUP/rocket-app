use crate::common::pojo::dto::request::query::UserPageQuery;
use crate::domain::table::user::User;

htmlsql_select_page!(select_page_by_params(params:&UserPageQuery)->User=>"sql/html/user.html");
