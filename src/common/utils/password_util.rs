use crate::security::encoder::delegating_password_encoder::DELEGATING_PASSWORD_ENCODER;
use crate::security::encoder::password_encoder::PasswordEncoder;
use std::sync::Arc;

const DEFAULT_PASSWORD: &str = "123456";

pub fn default_password() -> String {
    encode(DEFAULT_PASSWORD.to_string())
}

pub fn encode(raw_password: String) -> String {
    Arc::clone(&DELEGATING_PASSWORD_ENCODER).encode(raw_password)
}

pub fn matches(raw_password: &str, encoded_password: &str) -> bool {
    Arc::clone(&DELEGATING_PASSWORD_ENCODER).matches(raw_password, encoded_password)
}
