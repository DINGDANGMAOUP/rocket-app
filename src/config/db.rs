use std::time::Duration;

use rbatis::RBatis;

use crate::config::config::SystemConfig;

pub async  fn init_db(config: &SystemConfig)-> RBatis {
    let rb = RBatis::new();
    rb.init(rbdc_pg::driver::PgDriver {}, &config.app.datasource.url)
        .unwrap();
    let pool = rb.get_pool().unwrap();
    //max connections
    pool.set_max_open_conns(config.app.datasource.db_pool_len as u64).await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(
        config.app.datasource.db_pool_timeout as u64,
    ))).await;
    return rb;
}
