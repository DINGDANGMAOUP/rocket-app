pub trait PasswordEncoder {
    fn encode(&self, raw_password: String) -> String;
    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool;
    fn upgrade_encoding(&self, _encoded_password: &str) -> bool {
        false
    }
}
