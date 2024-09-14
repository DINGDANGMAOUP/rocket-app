use crate::domain::table::tables::{
    CommonTable, DictData, DictType, Menu, Role, RoleMenu, User, UserRole,
};
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
    //公共字段
    let common = CommonTable {
        id: Some(Default::default()),
        create_time: Some(Default::default()),
        update_time: Some(Default::default()),
        create_by: Some(Default::default()),
        update_by: Some(Default::default()),
        remark: Some(Default::default()),
        del_flag: Some(Default::default()),
    };
    let table = User {
        common: common.clone(),
        username: Some(Default::default()),
        password: Some(Default::default()),
        nick_name: Some(Default::default()),
        phone: Some(Default::default()),
        email: Some(Default::default()),
        sex: Some(Default::default()),
    };

    let _ = RBatis::sync(&conn, mapper, &table, "t_user").await;

    let table = Role {
        common: common.clone(),
        role_name: Some(Default::default()),
        authority: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "t_role").await;

    let table = UserRole {
        common: common.clone(),
        user_id: Default::default(),
        role_id: Default::default(),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "t_user_role").await;

    let table = Menu {
        common: common.clone(),
        menu_name: Some(Default::default()),
        parent_id: Some(Default::default()),
        order_num: Some(Default::default()),
        path: Some(Default::default()),
        component: Some(Default::default()),
        is_frame: Some(Default::default()),
        is_cache: Some(Default::default()),
        is_show: Some(Default::default()),
        status: Some(Default::default()),
        perms: Some(Default::default()),
        icon: Some(Default::default()),
        type_: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "t_menu").await;

    let table = RoleMenu {
        common: common.clone(),
        role_id: Default::default(),
        menu_id: Default::default(),
    };

    let _ = RBatis::sync(&conn, mapper, &table, "t_role_menu").await;

    let table = DictType {
        common: common.clone(),
        dict_name: None,
        dict_type: None,
        status: None,
    };
    let _ = RBatis::sync(&conn, mapper, &table, "t_dict_type").await;

    let table = DictData {
        common: common.clone(),
        dict_type_id: None,
        dict_label: None,
        dict_value: None,
        dict_sort: None,
    };
    let _ = RBatis::sync(&conn, mapper, &table, "t_dict_data").await;
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
            common: Default::default(),
            username: Some("admin".to_string()),
            password: Some("123456".to_string()),
            nick_name: Some("admin".to_string()),
            phone: Some("123456".to_string()),
            email: Some("223@qw.com".to_string()),
            sex: Some(1),
        },
        User {
            common: Default::default(),
            username: Some("user".to_string()),
            password: Some("123".to_string()),
            nick_name: Some("user".to_string()),
            phone: Some("123".to_string()),
            email: None,
            sex: Some(0),
        },
    ];
    let _ = User::insert_batch(&conn, &users, users.len() as u64).await;

    let menu = Menu {
        common: Default::default(),
        menu_name: Some("系统管理".to_string()),
        parent_id: Some(0),
        order_num: Some(0),
        path: None,
        component: Some("Layout".to_string()),
        is_frame: Some(0),
        is_cache: Some(0),
        is_show: Some(1),
        status: Some(1),
        perms: None,
        icon: None,
        type_: Some(crate::common::constants::menu_type::MenuType::Menu),
    };
    let _ = Menu::insert(&conn, &menu).await;
}
