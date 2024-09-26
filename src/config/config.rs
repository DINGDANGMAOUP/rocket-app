use crate::common::utils::resource_util::load_config;
use lazy_static::lazy_static;
use rbs::to_value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub port: usize,
    pub host: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogConfig {
    //log_level: off,info,error,warn,debug,trace
    pub level: String,
    //log dir, you can use "target/logs/abc.log"  default is "target/logs/"
    pub dir: String,
    //rolling by LogFileSize use[1KB,1MB,1GB], rolling by date use ['hour','minute','day'],rolling by Duration use ['1hour','10minute','1day']
    pub rolling: String,
    // Log rolling retention options:
    // Retain all logs: All
    // Retain logs by time (in seconds): KeepTime(i64)
    // Retain logs by version: KeepNum(i64)
    pub keep_type: String,
    // Optional log packaging formats: "" (keep as .log), "gzip" (GZip compressed), "zip" (ZIP compressed), "lz4" (LZ4 compressed (very fast))
    // The following options need to be enabled:
    // Inside the toml file, add to 'fast_log': fast_log = { version = "1.5", features = ["lz4", "zip", "gzip"]}
    // In src/config/log.rs, uncomment the section under fn choose_packer()
    // In application.yml, add: log_pack_compress: "zip"
    pub pack_compress: String,
    //Log channel length: null for unbounded queue, non-null for bounded queue (better performance)
    pub chan_len: usize,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub url: String,
    pub db_pool_len: u64,
    pub db_pool_timeout: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub prefix: String,
    pub expire: u64,
    pub issuer: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityConfig {
    pub white_list: Vec<String>,
    pub token: Token,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedisConfig {
    pub url: String,
    // pub db: u8,
    // pub pool_size: u8,
    // pub pool_timeout: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub debug: bool,
    pub datasource: Datasource,
    pub security: SecurityConfig,
    pub redis: RedisConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemConfig {
    pub server: Server,
    pub app: AppConfig,
    pub logging: LogConfig,
}

impl Default for SystemConfig {
    fn default() -> Self {
        let cfg_data = load_config().expect("read 'application.yml' fail");
        //load config
        let mut result: SystemConfig =
            serde_yml::from_str(&cfg_data).expect("load config file fail");
        if cfg!(debug_assertions) {
            result.app.debug = true;
        } else {
            result.app.debug = false;
        }
        if result.app.debug {
            println!("[app] {}", to_value!(&result));
            println!(
                "[app] ///////////////////// Start On Debug Mode ////////////////////////////"
            );
        } else {
            println!(
                "[app] ///////////////////// Start On Release Mode ////////////////////////////"
            );
        }
        result
    }
}
lazy_static! {
    pub static ref SYSTEM_CONFIG: SystemConfig = SystemConfig::default();
}
