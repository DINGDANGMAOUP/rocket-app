use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

const IP_QUERY_URL: &str = "http://whois.pconline.com.cn/ipJson.jsp";
const UNKNOWN: &str = "XX XX";
const LOCAL_IP: &str = "0:0:0:0:0:0:0:1";
#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub pro: String,
    pub city: String,
}
pub async fn get_ip_address(ip: &str) -> Result<String, Error> {
    if ip == LOCAL_IP || "127.0.0.1" == ip {
        return Ok("内网IP".to_string());
    }
    let url = format!("{}?ip={}&json=true", IP_QUERY_URL, ip);
    let resp = reqwest::get(&url).await;
    match resp {
        Ok(res) => {
            let obj = res.text().await.unwrap();
            let obj = serde_json::from_str::<Address>(&obj).unwrap();
            let region = obj.pro;
            let city = obj.city;
            Ok(format!("{} {}", region, city))
        }
        Err(..) => {
            log::error!("获取地理位置异常 {}", ip);
            Ok(UNKNOWN.to_string())
        }
    }
}
