use crate::domain::table::tables::{CommonTable, User};
use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::intercept_log::LogInterceptor;
use rbatis::table_sync::{
    ColumMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper,
};
use rbatis::RBatis;
pub async fn sync_tables(rb: &RBatis) {
    //disable log
    let log_intercept = rb.get_intercept::<LogInterceptor>().unwrap();
    let level = log_intercept.get_level_filter().clone();
    log_intercept.set_level_filter(LevelFilter::Off);
    defer!(|| {
        log_intercept.set_level_filter(level);
    });
    let mapper = {
        match rb.driver_type().unwrap() {
            "sqlite" => &SqliteTableMapper {} as &dyn ColumMapper,
            "mssql" => &MssqlTableMapper {} as &dyn ColumMapper,
            "mysql" => &MysqlTableMapper {} as &dyn ColumMapper,
            "postgres" => &PGTableMapper {} as &dyn ColumMapper,
            _ => {
                panic!("not find driver mapper")
            }
        }
    };
    let conn = rb.acquire().await.expect("connection database fail");
    let table = User {
        common: CommonTable{
            id: Some(Default::default()),
            create_time: Some(Default::default()),
            update_time: Some(Default::default()),
            create_by: Some(Default::default()),
            update_by: Some(Default::default()),
            remark: Some(Default::default()),
            del_flag: Some(Default::default()),
        },
        username: Some(Default::default()), 
        password: Some(Default::default()),
        nick_bame: Some(Default::default()),
        phone: Some(Default::default()),
        email: Some(Default::default()),
        sex: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "t_user").await;
}

pub async fn sync_tables_data(rb: &RBatis) {
    let conn = rb.acquire().await.expect("init data fail");
    if let Ok(v) = User::select_by_column(&conn, "id", 0).await {
        if v.len() > 0 {
            //if user exists,return
            return;
        }
    };
    let users = vec![
        User {
            common: CommonTable{
                id: Some(0),
                create_time: Some(Default::default()),
                update_time: Some(Default::default()),
                create_by: Some(Default::default()),
                update_by: Some(Default::default()),
                remark: Some(Default::default()),
                del_flag: Some(Default::default()),
            },
            username: Some("admin".to_string()),
            password: Some("123456".to_string()),
            nick_bame: Some("admin".to_string()),
            phone: Some("123456".to_string()),
            email: Some("223@qw.com".to_string()),
            sex: Some(1),

        },
        User {
            common: CommonTable{
                id: Some(1),
                create_time: Some(Default::default()),
                update_time: Some(Default::default()),
                create_by: Some(Default::default()),
                update_by: Some(Default::default()),
                remark: Some(Default::default()),
                del_flag: Some(Default::default()),
            },
            username: Some("user".to_string()),
            password: Some("123".to_string()),
            nick_bame: Some("user".to_string()), 
            phone: Some("123".to_string()),
            email: Some("".to_string()),
            sex: Some(0),
        },
    ];
    let _ = User::insert_batch(&conn, &users, users.len() as u64).await;
}
