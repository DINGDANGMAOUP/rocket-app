
use std::{fs::File, io::Read};

use rbs::to_value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server{
    port: usize,
    host: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct  Datasource{
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig{
    pub debug: bool,
    pub datasource: Datasource,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemConfig{ 
    pub server: Server,
    pub app: AppConfig,
}

impl Default for SystemConfig{
    fn default() -> Self{
        let mut f = File::open("application.yml").expect("not find 'application.yml'");
        let mut cfg_data = "".to_string();
        f.read_to_string(&mut cfg_data).expect("read 'application.yml' fail");
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
            println!("[app] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[app] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
    
}

mod test{






    #[test]
    fn test_config(){
        use std::fs::File;
        use std::io::Read;
        use crate::config::config::SystemConfig;

        let mut f = File::open("application.yml").expect("not find 'application.yml'");
        let mut cfg_data = "".to_string();
        f.read_to_string(&mut cfg_data).expect("read 'application.yml' fail");
        let data: SystemConfig = serde_yml::from_str(&cfg_data).expect("data must be valid");
        println!("{:?}",data);
    }

}