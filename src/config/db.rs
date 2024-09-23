use crate::common::plugins::returning_id_plugin::ReturningIdPlugin;
use crate::config::config::SystemConfig;
use rbatis::RBatis;
use std::{sync::Arc, time::Duration};

pub async fn init_db(config: &SystemConfig) -> RBatis {
    let rb = RBatis::new();
    rb.link(rbdc_pg::driver::PgDriver {}, &config.app.datasource.url)
        .await
        .unwrap();
    rb.intercepts.push(Arc::new(ReturningIdPlugin {}));
    // rb.intercepts.push(Arc::new(LogicDelPlugin {}));
    let pool = rb.get_pool().unwrap();
    //max connections
    pool.set_max_open_conns(config.app.datasource.db_pool_len)
        .await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(
        config.app.datasource.db_pool_timeout,
    )))
    .await;
    rb
}
