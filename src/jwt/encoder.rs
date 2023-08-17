use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;

pub struct JwtEncoder {
    encoding_key: EncodingKey,
}

impl JwtEncoder {
    pub fn new(private_key: Vec<u8>) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(Self {
            encoding_key: EncodingKey::from_rsa_pem(&private_key)?,
        })
    }

    pub fn encode<ClaimsModel: Serialize>(
        &self,
        claims: &ClaimsModel,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &self.encoding_key)
    }
}
