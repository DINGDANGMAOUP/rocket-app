use crate::security::encoder::password_encoder::PasswordEncoder;
#[deprecated(note = "use `BcryptPasswordEncoder` instead")]
pub struct NoOpPasswordEncoder;

impl PasswordEncoder for NoOpPasswordEncoder {
    fn encode(&self, raw_password: &str) -> String {
        raw_password.to_string()
    }

    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool {
        raw_password == encoded_password
    }
}
