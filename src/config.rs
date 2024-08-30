use std::{collections::HashMap, fs::File, io::Read};
use lazy_static::lazy_static;
use toml::Value;

const CONFIG_TOML_PATH: &str = "application.toml";
lazy_static! {
    pub static ref CONFIG: HashMap<String, Value> = {
        let config = init_config().unwrap();
        let mut map = HashMap::new();
        for (key, value) in config.as_table().unwrap() {
            map.insert(key.to_string(), value.clone());
        }
        map
    };
}
pub fn init_config() -> Option<Value> {
    // 打开TOML文件
    let file = File::open(CONFIG_TOML_PATH);
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            log::error!("Error opening file: {}", e);
            return None;
        }
    };
    // 读取文件内容
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    // 解析TOML字符串为Value
    let toml_value: Value = toml::from_str(&content).unwrap();
    Some(toml_value)
}

