use crate::common::constants::http_code;
use actix_web::HttpResponse;
use serde::{ser, Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Response;
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData<T> {
    pub success: bool,
    pub err_code: String,
    pub err_message: Option<String>,
    pub data: Option<T>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDesc {
    pub success: bool,
    pub err_code: String,
    pub err_message: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ResponseErr<T: ser::Serialize> {
    pub success: bool,
    pub err_code: String,
    pub err_message: Option<T>,
}

impl Response {
    pub fn build_success() -> HttpResponse {
        HttpResponse::Ok().json(ResponseDesc {
            success: true,
            err_code: String::from(http_code::SUCCESS),
            err_message: None,
        })
    }
    pub fn build_error(err_code: String, err_message: String) -> HttpResponse {
        HttpResponse::Ok().json(ResponseDesc {
            success: false,
            err_code,
            err_message: Some(err_message),
        })
    }

    pub fn build_data<T: ser::Serialize>(data: &T) -> HttpResponse {
        HttpResponse::Ok().json(ResponseData {
            success: false,
            err_code: String::from(http_code::SUCCESS),
            err_message: None,
            data: Some(data),
        })
    }
}
