use crate::domain::table::user::User;
use crate::pojo::dto::query::UserPageQuery;

htmlsql_select_page!(select_page_by_params(params:&UserPageQuery)->User=>"sql/html/user.html");