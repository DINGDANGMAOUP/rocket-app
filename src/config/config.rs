use std::{fs::File, io::Read};

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
    // In application.json5, add: log_pack_compress: "zip"
    pub pack_compress: String,
    //Log channel length: null for unbounded queue, non-null for bounded queue (better performance)
    pub chan_len: usize,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub url: String,
    pub db_pool_len: usize,
    pub db_pool_timeout: usize,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub debug: bool,
    pub datasource: Datasource,
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
        let mut f = File::open("application.yml").expect("not find 'application.yml'");
        let mut cfg_data = "".to_string();
        f.read_to_string(&mut cfg_data)
            .expect("read 'application.yml' fail");
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

mod test {

    #[test]
    fn test_config() {
        use crate::config::config::SystemConfig;
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("application.yml").expect("not find 'application.yml'");
        let mut cfg_data = "".to_string();
        f.read_to_string(&mut cfg_data)
            .expect("read 'application.yml' fail");
        let data: SystemConfig = serde_yml::from_str(&cfg_data).expect("data must be valid");
        println!("{:?}", data);
    }
}
