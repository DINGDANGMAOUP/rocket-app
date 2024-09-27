use crate::common::utils::address_util;
use crate::common::utils::jwt_util::parse_jwt;
use crate::config::config::SYSTEM_CONFIG;
use crate::response::ResponseDesc;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use serde_json::json;
use std::future::{ready, Future, Ready};
use std::pin::Pin;

const AUTH_HEADER: &str = "Authorization";

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
        let remote_ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap()
            .to_owned();
        let path = req.path().to_owned();
        let headers = req.headers().to_owned();
        let fut = self.service.call(req);
        Box::pin(async move {
            let remote_ip = remote_ip;
            log::debug!("remote_ip :{:?}", remote_ip);
            let address = address_util::get_ip_address(&remote_ip).await.unwrap();
            log::debug!("address :{:?}", address);
            let config = &SYSTEM_CONFIG;
            let security_config = config.app.security.clone();
            //判断是否白名单 如果是白名单则直接放行
            let white_list = security_config.white_list.clone();
            let mut is_white = false;
            let path = path;
            for white in white_list {
                if path.starts_with(&white) {
                    is_white = true;
                    break;
                }
            }
            if is_white {
                let res = fut.await?;
                println!("Hi from response");
                return Ok(res);
            }
            let headers = headers;
            let token = match headers.get(AUTH_HEADER) {
                Some(token_value) => token_value,
                None => {
                    return Err(actix_web::error::ErrorUnauthorized(json!(ResponseDesc {
                        success: false,
                        err_code: String::from("401"),
                        err_message: Some("The token cannot be null".to_string()),
                    })));
                }
            };
            log::debug!("header :{:?}", token);
            let token = match token.to_str() {
                Ok(token) => token,
                Err(_) => {
                    return Err(actix_web::error::ErrorUnauthorized(json!(ResponseDesc {
                        success: false,
                        err_code: String::from("402"),
                        err_message: Some("The token cannot be null".to_string()),
                    })));
                }
            };
            let mut token = &*token.replace(&security_config.token.prefix, "");
            token = token.trim();
            log::debug!("token :{}", token);
            let token = parse_jwt(token);
            let claims = match token {
                Ok(claims) => claims,
                Err(e) => {
                    return Err(actix_web::error::ErrorUnauthorized(json!(ResponseDesc {
                        success: false,
                        err_code: String::from("403"),
                        err_message: Some(e.to_string()),
                    })));
                }
            };
            log::debug!("parse jwt:{:?}", claims);
            let res = fut.await?;
            println!("Hi from response");
            Ok(res)
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::common::utils::jwt_util::Claims;
    use crate::common::utils::resource_util::load_secret;
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
    use std::collections::HashMap;

    #[actix_web::test]
    async fn test_index_get() {
        let my_claims = Claims {
            aud: None,
            sub: Option::from("dzhao".to_string()),
            exp: 10000000000,
            iat: None,
            iss: None,
            nbf: None,
        };
        let private_key = load_secret("private.pem").unwrap();
        let token = encode(
            &Header::new(Algorithm::RS256),
            &my_claims,
            &EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap(),
        )
        .unwrap();
        println!("token:{}", token);
        let public_key = load_secret("public.pem").unwrap();
        let token_str = decode::<HashMap<String, serde_json::Value>>(
            &token,
            &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
            &Validation::new(Algorithm::RS256),
        );
        println!("token_str:{:?}", token_str);
    }
}
