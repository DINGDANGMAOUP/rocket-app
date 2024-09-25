use crate::error::Error;
use rbatis::rbdc::Json;
use regex::{Match, Matches};
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

const IP_QUERY_URL: &str = "http://whois.pconline.com.cn/ipJson.jsp";
const UNKNOWN: &str = "XX XX";
const LOCAL_IP: &str = "0:0:0:0:0:0:0:1";
#[derive(Debug, Deserialize, Serialize)]
struct Address {
    pub pro: String,
    pub city: String,
}
pub async fn get_ip_address(ip: &str) -> Result<String, Error> {
    if ip == LOCAL_IP || is_inner_ip(ip) {
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
//^(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)$

fn is_inner_ip(ip: &str) -> bool {
    let re = regex::Regex::new(r"^(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)$").unwrap();
    re.is_match(ip)
}

#[test]
fn test() -> Result<(), ReqwestError> {
    let ip = is_inner_ip("127.0.0.1");
    println!("ip:{}", ip);
    // let response = reqwest::blocking::get(
    //     "http://whois.pconline.com.cn/ipJson.jsp?ip=117.25.169.123&json=true",
    // );
    // let obj = response?.text()?;
    // let objStr = serde_json::from_str::<Address>(&obj).unwrap();
    // println!("obj:{:?}", json!(&objStr));
    // let string = get_ip_address("117.25.169.123").await.unwrap();
    // println!("string:{}", string);
    Ok(())
}
