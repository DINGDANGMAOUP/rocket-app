use crate::common::pojo::dto::request::query::UserPageQuery;
use crate::domain::table::user::User;

htmlsql_select_page!(select_page_by_params(params:&UserPageQuery)->User=>"sql/html/user.html");
impl_select!(User{select_by_username(table_column:&str,username:&str) -> Option=>"`where username = #{username} limit 1`"},"t_user");
