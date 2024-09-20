use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::collections::{HashMap, HashSet};
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
        println!("Hi from start. You requested: {}", req.path());
        println!("Hi from start. You requested: {:?}", req);
        let headers = req.headers();
        let token = headers.get("Authorization").unwrap();
        println!("header :{:?}", token);
        let tk = token.to_str().unwrap().replace("Bearer ", "");
        println!("tk :{}", tk);
        let validation = {
            let mut validation = Validation::default();
            validation.validate_exp = false;
            validation.validate_aud = false;
            validation.validate_nbf = false;
            validation
        };
        let token = decode::<HashMap<String, serde_json::Value>>(
            &tk,
            &DecodingKey::from_secret("tooltt".as_ref()),
            &validation,
        );
        println!("token:{:?}", token);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            println!("Hi from response");
            Ok(res)
        })
    }
}
