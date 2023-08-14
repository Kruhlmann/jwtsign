mod error;

// openssl rsa -pubin \
//             -in public_key.pem \
//             -inform PEM \
//             -RSAPublicKey_out \
//             -outform DER \
//             -out public_key.der

use error::InvalidJWTError;
use jsonwebtoken::{jwk::Jwk, Header, Algorithm};
use ring::{signature, rand};
use ring::signature::RsaKeyPair;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose, DecodeError};
use serde::Serialize;

const JWT_HEADER_INDEX: usize = 0;
const JWT_PAYLOAD_INDEX: usize = 1;
const JWT_SIGNATURE_INDEX: usize = 2;

#[derive(Debug, PartialEq)]
pub struct JWT {
    header: Header,
    payload: serde_json::Value,
    secret: String,
    signed: bool,
}

#[derive(Serialize, Debug)]
pub struct SerializableJWTHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    pub alg: Algorithm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwk: Option<Jwk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5u: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5c: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5t_s256: Option<String>,
}

impl SerializableJWTHeader {
    pub fn from_external(header: Header) -> Self {
        SerializableJWTHeader {
            typ: header.typ,
            alg: header.alg,
            cty: header.cty,
            jku: header.jku,
            jwk: header.jwk,
            kid: header.kid,
            x5u: header.x5u,
            x5c: header.x5c,
            x5t: header.x5t,
            x5t_s256: header.x5t_s256,
        }
    }
}

impl JWT {
    fn base64_decode_utf8_lossy(base64_str: &str) -> Result<String, DecodeError> {
        match &general_purpose::STANDARD_NO_PAD.decode(base64_str) {
            Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
            Err(e) => Err(e.clone()),
        }
        
    }

    pub fn from_token(jwt_token: &str) -> Result<Self, InvalidJWTError> {
        let parts: Vec<&str> = jwt_token.split('.').collect();

        if parts.len() == 3 {
            let header_str = Self::base64_decode_utf8_lossy(parts[JWT_HEADER_INDEX]).unwrap();
            let payload_str = Self::base64_decode_utf8_lossy(parts[JWT_PAYLOAD_INDEX]).unwrap();
            let header = serde_json::from_str(&header_str).unwrap();
            let payload = serde_json::from_str(&payload_str).unwrap();
            let secret = parts[JWT_SIGNATURE_INDEX].to_owned();
            let signed = !secret.is_empty();
            Ok(JWT { header, payload, secret, signed })
        } else {
            Err(InvalidJWTError::new("JWT token must contain 3 parts"))
        }
    }

    pub fn to_token(&self) -> Result<String, InvalidJWTError> {
        let serializable_header = SerializableJWTHeader::from_external(self.header.clone());
        let header_str = serde_json::to_string(&serializable_header).unwrap();
        let payload_str = serde_json::to_string(&self.payload).unwrap();
        let header_encoded = &general_purpose::URL_SAFE_NO_PAD.encode(header_str);
        let payload_encoded = general_purpose::URL_SAFE_NO_PAD.encode(payload_str);

        Ok(format!("{}.{}.{}", header_encoded, payload_encoded, self.secret))
    }

    pub fn rsa_pkcs1_sha256_sign(&mut self, private_key: &str) -> Result<(), InvalidJWTError> {
        let base64_header = "";
        let base64_body = "";
        let message = format!("{}.{}", base64_header, base64_body);
        let key_pair = RsaKeyPair::from_der(private_key.as_bytes()).map_err(|_| InvalidJWTError { message: "invalid".to_string() })?;
        let rng = rand::SystemRandom::new();
        let mut signature = vec![0; key_pair.public_modulus_len()];
        let _ = key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, message.as_bytes(), &mut signature);
        let encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(&signature);
        self.secret = encoded_signature;
        self.signed = true;
        Ok(())
    }
}

// pub fn decode_jwt(token: &str, public_key: &str) -> Result<serde_json::Value, jsonwebtoken::errors::Error> {
//     let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
//     let validation = Validation::new(Algorithm::RS256);
//     let decoded = jsonwebtoken::decode::<serde_json::Value>(token, &decoding_key, &validation)?;

//     Ok(decoded.claims)
// }

// pub fn verify_jwt(token: &str, public_key: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
//     let claims = decode_jwt(token, public_key)?;
//     let current_time = SystemTime::now();
//     let unix_timestamp = current_time
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards")
//         .as_secs();
//     true

    // match claims.get("exp") {

    // }
    // if claims.exp < jsonwebtoken::dangerous_unsafe_time::now_utc() as usize {
    //     Err(jsonwebtoken::errors::ErrorKind::ExpiredSignature.into())
    // } else {
    //     Ok(claims)
    // }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_token_and_to_token() -> Result<(), InvalidJWTError> {
        let jwt_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let jwt = JWT::from_token(jwt_token)?;
        let converted_token = jwt.to_token()?;
        let converted_jwt = JWT::from_token(&converted_token)?;
        assert_eq!(jwt, converted_jwt);
        Ok(())
    }

    fn test_signature() -> Result<(), InvalidJWTError> {
        let jwt_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.";
        let der_key = "3082025a02010002818100f22f9e033f70d9f3b1a343b301790e12f7daaa1d44d793a80138d1b4f0e1b61334a8e0c63da40c8440ed51d2adab365ce4b8db3c239b0da4f74bb0fcd98a744e6d2c1fbf80f6f89360da31ebcaae10c7fb1e225a7032bd0d3f1b31c4852a7b1a55d92b8ff9d76b8e529af70ea2e7f0e1b26b0e333c3a0203010001028180087bf151dbb10ac484a26f066c5333bdeea7a6c73f15a0af5b5a31cb5677f2657265c5b2c878f39ab4cbfe47c5e5d79fa05df1c38f2fe39ad8614536891ba18c20ba4d89869f708a846e6769bcf1f91102fbb18bcb63a0df73ceab2077f828fb8da0233c07a1d5b56e1a2837a1686c5751a1f96a0f3ea9f981c73e2fcfeb022100c2f9e033f70d9f3b1a343b301790e12f7daaa1d44d793a80138d1b4f0e1b61302022100f22f9e033f70d9f3b1a343b301790e12f7daaa1d44d793a80138d1b4f0e1b61302022100f22f9e033f70d9f3b1a343b301790e12f7daaa1d44d793a80138d1b4f0e1b61302022100f22f9e033f70d9f3b1a343b301790e12f7daaa1d44d793a80138d1b4f0e1b61302022100f22f9e033f70d9f3b1a343b301790e12f7daaa1d44d793a80138d1b4f0e1b61302022100f22f9e033f70d9f3b1a343b301790e12f7daaa1d44d793a80138d1b4f0e1b613";
        let mut jwt = JWT::from_token(jwt_token)?;
        assert_eq!(jwt.signed, false);
        jwt.rsa_pkcs1_sha256_sign(der_key);
        assert_eq!(jwt.signed, true);
        
        Ok(())
    }
}
