pub trait PasswordEncoder {
    fn encode(&self, raw_password: &str) -> String;
    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool;
    fn upgrade_encoding(&self, encoded_password: &str) -> bool;
}
