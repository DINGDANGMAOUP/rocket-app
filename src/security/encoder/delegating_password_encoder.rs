use crate::security::encoder::password_encoder::PasswordEncoder;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

const ID_PREFIX: &str = "{";
const ID_SUFFIX: &str = "}";

/**
 * 委派密码编码器   对org.springframework.security.crypto.password.DelegatingPasswordEncoder的rust简化实现
 * @constructor 创建[DelegatingPasswordEncoder]
 * @param [idForEncode] 用于编码 ID
 * @param [idToPasswordEncoder] ID 到密码编码器
 * @param [idPrefix] id 前缀
 * @param [idSuffix] ID 后缀
 */

pub struct DelegatingPasswordEncoder {
    id_for_encode: String,
    id_to_password_encoder: HashMap<String, Arc<dyn PasswordEncoder + Sync + Send>>,
    id_prefix: String,
    id_suffix: String,
}

impl Default for DelegatingPasswordEncoder {
    fn default() -> Self {
        let mut encoder: HashMap<String, Arc<dyn PasswordEncoder + Sync + Send>> = HashMap::new();
        encoder.insert(
            "bcrypt".to_string(),
            Arc::new(crate::security::encoder::bcrypt_password_encoder::BcryptPasswordEncoder),
        );
        encoder.insert(
            "noop".to_string(),
            Arc::new(crate::security::encoder::no_op_password_encoder::NoOpPasswordEncoder),
        );
        DelegatingPasswordEncoder::new("bcrypt".to_string(), encoder)
    }
}

impl DelegatingPasswordEncoder {
    pub fn new(
        id_for_encode: String,
        id_to_password_encoder: HashMap<String, Arc<dyn PasswordEncoder + Sync + Send>>,
    ) -> Self {
        DelegatingPasswordEncoder {
            id_for_encode,
            id_to_password_encoder,
            id_prefix: ID_PREFIX.to_string(),
            id_suffix: ID_SUFFIX.to_string(),
        }
    }

    fn extract_id(&self, prefix_encoded_password: &str) -> Option<String> {
        //截取{}中的id
        let start = prefix_encoded_password.find(&self.id_prefix);
        if start != Some(0) {
            return None;
        }
        let end = prefix_encoded_password.find(&self.id_suffix);
        if end < Some(0) {
            return None;
        }
        Some(
            prefix_encoded_password[start.unwrap() + self.id_prefix.len()..end.unwrap()]
                .to_string(),
        )
    }

    fn extract_encoded_password(&self, prefix_encoded_password: &str) -> String {
        let start = prefix_encoded_password.find(&self.id_suffix);
        prefix_encoded_password[start.unwrap() + self.id_suffix.len()..].to_string()
    }
}

impl PasswordEncoder for DelegatingPasswordEncoder {
    fn encode(&self, raw_password: String) -> String {
        format!(
            "{}{}{}{}",
            self.id_prefix,
            self.id_for_encode,
            self.id_suffix,
            self.id_to_password_encoder
                .get(&self.id_for_encode)
                .unwrap()
                .encode(raw_password)
        )
    }

    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool {
        let id = self
            .extract_id(encoded_password)
            .expect("No id found in encoded password");
        let delegate = match self.id_to_password_encoder.get(&id) {
            Some(delegate) => delegate.as_ref(),
            None => self
                .id_to_password_encoder
                .get(&self.id_for_encode)
                .expect("No DefaultPasswordEncoder found for id")
                .as_ref(),
        };
        let encoded_password = self.extract_encoded_password(encoded_password);
        delegate.matches(raw_password, &encoded_password)
    }

    fn upgrade_encoding(&self, encoded_password: &str) -> bool {
        let id = self
            .extract_id(encoded_password)
            .expect("No id found in encoded password");
        if !self.id_for_encode.eq_ignore_ascii_case(&id) {
            return true;
        }
        let encoded_password = self.extract_encoded_password(encoded_password);
        match self.id_to_password_encoder.get(&id) {
            Some(delegate) => delegate.upgrade_encoding(&encoded_password),
            None => false,
        }
    }
}

lazy_static! {
    pub static ref DELEGATING_PASSWORD_ENCODER: Arc<DelegatingPasswordEncoder> =
        Arc::new(DelegatingPasswordEncoder::default());
}
