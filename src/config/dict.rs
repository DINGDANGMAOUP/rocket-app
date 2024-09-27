use crate::config::redis::set_redis_map;
use crate::error::Error;
use crate::mapper::dict_mapper;
use rbatis::RBatis;
use std::collections::HashMap;

pub async fn init_dict(rb: &RBatis) -> Result<(), Error> {
    let vec = dict_mapper::select_dict_data(rb).await?;
    //对数据进行dict_name分组 hashmap key:dict_name value:hashmap key:dict_label value:dict_value
    let mut dict_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    for dict in vec {
        let dict_name = dict.dict_name.unwrap().clone();
        let dict_label = dict.dict_label.unwrap().clone();
        let dict_value = dict.dict_value.unwrap().clone();
        if dict_map.contains_key(&dict_name) {
            let map = dict_map.get_mut(&dict_name).unwrap();
            map.insert(dict_label, dict_value);
        } else {
            let mut dict_label_map = HashMap::new();
            dict_label_map.insert(dict_label, dict_value);
            dict_map.insert(dict_name, dict_label_map);
        }
    }
    //存入redis
    for (key, value) in dict_map {
        let key = format!("dict:{}", key);
        set_redis_map(&key, value)?;
    }

    Ok(())
}
