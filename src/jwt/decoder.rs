use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;

pub struct JwtDecoder {
    decoding_key: DecodingKey,
    validator: Validation,
}

impl JwtDecoder {
    pub fn new(public_key: Vec<u8>, leeway: u64) -> Result<Self, jsonwebtoken::errors::Error> {
        let mut validator = Validation::new(Algorithm::RS256);
        validator.leeway = leeway;
        Ok(Self {
            decoding_key: DecodingKey::from_rsa_pem(&public_key)?,
            validator,
        })
    }

    pub fn decode<ClaimsModel: DeserializeOwned>(
        &self,
        token: String,
    ) -> Result<ClaimsModel, jsonwebtoken::errors::Error> {
        Ok(
            jsonwebtoken::decode::<ClaimsModel>(&token, &self.decoding_key, &self.validator)?
                .claims,
        )
    }
}
