use crate::config::config::SYSTEM_CONFIG;
use crate::response::ResponseDesc;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::err;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use std::collections::HashMap;
use std::future::{ready, Future, Ready};
use std::pin::Pin;

pub struct JWTFilter;

impl<S, B> Transform<S, ServiceRequest> for JWTFilter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JWTMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTMiddleware { service }))
    }
}

pub struct JWTMiddleware<S> {
    service: S,
}
impl<S, B> Service<ServiceRequest> for JWTMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("Hi from start. You requested: {:?}", req);
        let config = &SYSTEM_CONFIG;
        let security_config = config.app.security.clone();
        //判断是否白名单 如果是白名单则直接放行
        let white_list = security_config.white_list.clone();
        let path = req.path();
        let mut is_white = false;
        for white in white_list {
            if path.starts_with(&white) {
                is_white = true;
                break;
            }
        }
        if is_white {
            return Box::pin(self.service.call(req));
        }

        let headers = req.headers();
        let token = {
            match headers.get("Authorization") {
                Some(token) => token,
                None => {
                    return Box::pin(err(actix_web::error::ErrorUnauthorized(json!(
                        ResponseDesc {
                            success: false,
                            err_code: String::from("401"),
                            err_message: Some("Unauthorized".to_string()),
                        }
                    ))));
                }
            }
        };
        log::info!("header :{:?}", token);
        let tk = token.to_str().unwrap().replace("Bearer ", "");
        log::info!("tk :{}", tk);
        let validation = {
            let mut validation = Validation::default();
            validation.validate_exp = false;
            validation.validate_aud = false;
            validation.validate_nbf = false;
            validation
        };
        let token = decode::<HashMap<String, serde_json::Value>>(
            &tk,
            &DecodingKey::from_secret(security_config.secret.as_ref()),
            &validation,
        );
        log::info!("token:{:?}", token);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            log::info!("Hi from response");
            Ok(res)
        })
    }
}
