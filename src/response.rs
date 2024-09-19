use crate::common::constants::http_code;
use actix_web::HttpResponse;
use serde::{ser, Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData<T> {
    pub success: bool,
    pub err_code: String,
    pub err_message: Option<String>,
    pub data: Option<T>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePage<T> {
    pub success: bool,
    pub err_code: String,
    pub err_message: Option<String>,
    pub data: Option<T>,
    pub total: Option<u64>,
    pub page_size: Option<u64>,
    pub page_No: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDesc {
    pub success: bool,
    pub err_code: String,
    pub err_message: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
            success: true,
            err_code: String::from(http_code::SUCCESS),
            err_message: None,
            data: Some(data),
        })
    }
    pub fn build_page<T:ser::Serialize>(data:&T,total:u64,page_size:u64,page_no:u64)->HttpResponse{
        HttpResponse::Ok().json(ResponsePage{
            success:true,
            err_code:String::from(http_code::SUCCESS),
            err_message:None,
            data:Some(data),
            total:Some(total),
            page_size:Some(page_size),
            page_No:Some(page_no),
        })
    }
}
