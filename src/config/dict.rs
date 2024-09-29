use crate::config::redis::{get_redis_map, set_redis_map};
use crate::error::Error;
use crate::mapper::dict_mapper;
use rbatis::RBatis;
use serde_json::json;
use std::collections::HashMap;

pub async fn init_dict(rb: &RBatis) -> Result<(), Error> {
    let vec = dict_mapper::select_dict_data(rb).await?;
    //对数据进行dict_name分组 hashmap key:dict_name value:hashmap key:dict_label value:dict_value
    let mut dict_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    for dict in vec {
        let dict_name = dict.dict_name.unwrap();
        let dict_label = dict.dict_label.unwrap();
        let dict_value = dict.dict_value.unwrap();
        if let std::collections::hash_map::Entry::Vacant(e) = dict_map.entry(dict_name.to_owned()) {
            let mut dict_label_map = HashMap::new();
            dict_label_map.insert(dict_label, dict_value);
            e.insert(dict_label_map);
        } else {
            let map = dict_map.get_mut(&dict_name).unwrap();
            map.insert(dict_label, dict_value);
        }
    }
    //存入redis
    for (key, value) in dict_map {
        let key = format!("dict:{}", key);
        set_redis_map(&key, value)?;
    }

    Ok(())
}
pub async fn get_dict(rb: &RBatis, dict_name: &str) -> Result<HashMap<String, String>, Error> {
    let key = format!("dict:{}", dict_name);
    let a = vec!["1", "2", "3"];
    let map: HashMap<String, String> = get_redis_map(&key, a)?;

    Ok(map)
}
