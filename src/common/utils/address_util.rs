use crate::error::Error;
use std::collections::HashMap;

const IP_QUERY_URL: &str = "http://whois.pconline.com.cn/ipJson.jsp";
const UNKNOWN: &str = "XX XX";
const LOCAL_IP: &str = "0:0:0:0:0:0:0:1";

pub async fn get_ip_address(ip: &str) -> Result<String, Error> {
    if ip == LOCAL_IP {
        return Ok("内网IP".to_string());
    }
    let url = format!("{}?ip={}&json=true", IP_QUERY_URL, ip);
    let resp = reqwest::get(&url).await;
    match resp {
        Ok(res) => {
            let obj = res
                .json::<HashMap<String, serde_json::Value>>()
                .await
                .unwrap();
            let region = obj["pro"].as_str().unwrap();
            let city = obj["city"].as_str().unwrap();
            Ok(format!("{} {}", region, city))
        }
        Err(..) => {
            log::error!("获取地理位置异常 {}", ip);
            Ok(UNKNOWN.to_string())
        }
    }
}
//^(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)$
