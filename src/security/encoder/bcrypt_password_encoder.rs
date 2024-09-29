use crate::security::encoder::password_encoder::PasswordEncoder;
use rbatis::rbatis_codegen::ops::AsProxy;
const DEFAULT_COST: u32 = bcrypt::DEFAULT_COST;
const STRENGTH: u32 = 10;
pub struct BcryptPasswordEncoder;
impl PasswordEncoder for BcryptPasswordEncoder {
    fn encode(&self, raw_password: String) -> String {
        encrypt(raw_password, DEFAULT_COST)
    }

    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool {
        bcrypt::verify(raw_password, encoded_password)
            .unwrap()
            .bool()
    }

    fn upgrade_encoding(&self, encoded_password: &str) -> bool {
        if encoded_password.is_empty() {
            return false;
        }
        let re = regex::Regex::new(r"\\A\\$2([ayb])?\\$(\\d\\d)\\$[./0-9A-Za-z]{53}").unwrap();
        if re.is_match(encoded_password) {
            let strength = encoded_password.split("$").collect::<Vec<&str>>()[2]
                .parse::<u32>()
                .unwrap();
            strength < STRENGTH
        } else {
            false
        }
    }
}

fn encrypt(password: String, cost: u32) -> String {
    bcrypt::hash(password, cost).expect("Failed to encrypt password")
}
