use crate::pojo;
use actix_web::{HttpResponse,ResponseError};
use failure::Fail;
use pojo::dto::response::ResponseErr;
use rbatis::Error as RBError;
use serde_json::{json, Value as JsonValue};
use validator::ValidationErrors;

#[derive(Debug, Fail)]
pub enum Error {
    //401
    #[fail(display = "Unauthoried: {}", _0)]
    Unauthorized(JsonValue),

    // //403
    // #[fail(display = "Forbidden: {}", _0)]
    // Forbidden(JsonValue),

    // //404
    // #[fail(display = "Not Found: {}", _0)]
    // NotFound(JsonValue),

    // 422
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    //500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Unauthorized(e) => HttpResponse::Unauthorized().json(ResponseErr {
                success: false,
                err_code: "401".to_string(),
                err_message: Some(e.clone()),
            }),
            // Error::Forbidden(e) => HttpResponse::Forbidden().json(JsonErr {
            //     code: 403,
            //     message: Some(e.clone())
            // }),
            // Error::NotFound(e) => HttpResponse::NotFound().json(JsonErr {
            //     code: 404,
            //     message: Some(e.clone())
            // }),
            Error::UnprocessableEntity(e) => {
                HttpResponse::UnprocessableEntity().json(ResponseErr {
                    success: false,
                    err_code: "422".to_string(),
                    err_message: Some(e.clone()),
                })
            }
            Error::InternalServerError => HttpResponse::InternalServerError().json(ResponseErr {
                success: false,
                err_code: "500".to_string(),
                err_message: Some(json!("Internal Server Error!".to_string())),
            }),
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        let mut err = JsonValue::default();

        for (_, e) in e.field_errors().iter() {
            let e: Vec<JsonValue> = e.iter().map(|e| json!(e.message)).collect();
            err = json!(e);
        }

        Error::UnprocessableEntity(json!(err))
    }
}

impl From<RBError> for Error {
    fn from(e: RBError) -> Self {
        Error::UnprocessableEntity(json!(e.to_string()))
    }
}
