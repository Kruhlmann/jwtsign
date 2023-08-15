use jsonwebtoken::{Algorithm, Header};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct JwtEncoderDecoder {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validator: Validation,
}

impl JwtEncoderDecoder {
    pub fn new(
        private_key: Vec<u8>,
        public_key: Vec<u8>,
        leeway: u64,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        let mut validator = Validation::new(Algorithm::RS256);
        validator.leeway = leeway;
        Ok(Self {
            encoding_key: EncodingKey::from_rsa_pem(&private_key)?,
            decoding_key: DecodingKey::from_rsa_pem(&public_key)?,
            validator,
        })
    }

    pub fn encode<ClaimsModel: Serialize>(
        &self,
        claims: &ClaimsModel,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &self.encoding_key)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[derive(PartialEq, Deserialize, Serialize, Debug)]
    struct Claims {
        id: String,
        exp: u64,
    }

    #[test]
    fn test_encode_decode() -> Result<(), Box<dyn std::error::Error>> {
        let leeway: u64 = 60;
        let unixnow = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let claims = Claims {
            id: "aa34be0fa7".to_string(),
            exp: unixnow,
        };
        let private_key = std::fs::read("res/private_key.pem").unwrap();
        let public_key = std::fs::read("res/public_key.pem").unwrap();
        let encoder = JwtEncoderDecoder::new(private_key, public_key, leeway)?;
        let token = encoder.encode::<Claims>(&claims)?;
        let decoded_claims = encoder.decode::<Claims>(token)?;
        assert_eq!(claims, decoded_claims);
        Ok(())
    }

    #[test]
    fn test_encode_decode_expired() -> Result<(), Box<dyn std::error::Error>> {
        let leeway: u64 = 0;
        let unixnow = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let claims = Claims {
            id: "aa34be0fa7".to_string(),
            exp: unixnow,
        };
        let private_key = std::fs::read("res/private_key.pem").unwrap();
        let public_key = std::fs::read("res/public_key.pem").unwrap();
        let encoder = JwtEncoderDecoder::new(private_key, public_key, leeway)?;
        let token = encoder.encode::<Claims>(&claims)?;
        std::thread::sleep(Duration::from_secs(1)); // Sleep for 1 second to ensure the token is
                                                    // expired
        let decoded_claims = encoder.decode::<Claims>(token);
        assert!(
            decoded_claims.is_err(),
            "Decoding should have resulted in an error, but it succeeded."
        );
        Ok(())
    }

    #[test]
    fn test_encode_decode_invalid_public_key() {
        let private_key = std::fs::read("res/private_key.pem").unwrap();
        let public_key = std::fs::read("res/public_key.invalid.pem").unwrap();
        let encoder = JwtEncoderDecoder::new(private_key, public_key, 0);
        assert!(
            encoder.is_err(),
            "Encoder constructor should have resulted in an error, but it succeeded."
        );
    }

    #[test]
    fn test_encode_decode_invalid_private_key() {
        let private_key = std::fs::read("res/private_key.invalid.pem").unwrap();
        let public_key = std::fs::read("res/public_key.pem").unwrap();
        let encoder = JwtEncoderDecoder::new(private_key, public_key, 0);
        assert!(
            encoder.is_err(),
            "Encoder constructor should have resulted in an error, but it succeeded."
        );
    }
}
