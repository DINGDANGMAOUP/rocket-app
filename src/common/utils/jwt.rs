use crate::common::utils::resource::load_secret;
use crate::config::config::SYSTEM_CONFIG;
use crate::error::Error;
use chrono::Local;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: Option<String>, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: Option<usize>, // Optional. Issued at (as UTC timestamp)
    pub iss: Option<String>, // Optional. Issuer
    pub nbf: Option<usize>, // Optional. Not Before (as UTC timestamp)
    pub sub: Option<String>, // Optional. Subject (whom token refers to)
}
impl Default for Claims {
    fn default() -> Self {
        let expire = SYSTEM_CONFIG.app.security.token.expire;
        //生成过期时间戳
        let exp = Local::now().timestamp() as usize + expire as usize;
        Claims {
            aud: None,
            exp,
            iat: Some(Local::now().timestamp() as usize),
            iss: None,
            nbf: Some(Local::now().timestamp() as usize),
            sub: None,
        }
    }
}
#[derive(Debug)]
enum GrantType {
    AccessToken,
    RefreshToken,
}
impl Display for GrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrantType::AccessToken => write!(f, "access_token"),
            GrantType::RefreshToken => write!(f, "refresh_token"),
        }
    }
}
impl From<String> for GrantType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "access_token" => GrantType::AccessToken,
            "refresh_token" => GrantType::RefreshToken,
            _ => panic!("Invalid grant type"),
        }
    }
}
impl From<GrantType> for String {
    fn from(g: GrantType) -> Self {
        match g {
            GrantType::AccessToken => "access_token".to_string(),
            GrantType::RefreshToken => "refresh".to_string(),
        }
    }
}

pub fn gen_jwt(username: &str, grant_type: GrantType) -> Result<String, Error> {
    let mut claims = Claims::default();
    claims.aud = Some(username.to_string());
    claims.sub = Some(grant_type.into());
    let token = encode(&HEADER, &claims, &PRIVATE_KEY);
    Ok(token?)
}

pub fn parse_jwt(token: &str) -> Result<Claims, Error> {
    let token_str = decode::<Claims>(&token, &PUBLIC_KEY, &VALIDATION);
    Ok(token_str?.claims)
}

lazy_static! {
    static ref PRIVATE_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(load_secret("private.pem").unwrap().as_bytes()).unwrap();
    static ref PUBLIC_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(load_secret("public.pem").unwrap().as_bytes()).unwrap();
    static ref VALIDATION: Validation = {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        validation.validate_aud = true;
        validation.validate_nbf = true;
        validation
    };
    static ref HEADER: Header = Header::new(Algorithm::RS256);
}
