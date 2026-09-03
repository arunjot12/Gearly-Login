use super::claims::Claims;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::env;

#[derive(Clone)]
pub struct JwtService {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub access_token_till: Duration,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT service not found");
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_till: Duration::minutes(90),
        }
    }

    pub fn create_token(
        &self,
        user_id: i32,
        role: String,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, role, self.access_token_till);
        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
