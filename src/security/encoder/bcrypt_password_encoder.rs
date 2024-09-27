use crate::security::encoder::password_encoder::PasswordEncoder;

struct BcryptPasswordEncoder;
impl PasswordEncoder for BcryptPasswordEncoder {
    fn encode(&self, raw_password: &str) -> String {
        let salt = bcrypt::hash(raw_password, 4).unwrap();
        salt
    }

    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool {
        bcrypt::verify(raw_password, encoded_password).unwrap()
    }

    fn upgrade_encoding(&self, encoded_password: &str) -> bool {
        false
    }
}

fn encrypt(password: &str) -> String {
    "".to_string()
}
// fn encrypt(password: &str, salt: &str) -> String {
//     "".to_string()
// }
