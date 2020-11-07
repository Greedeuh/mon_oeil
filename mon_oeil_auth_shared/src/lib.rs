use actix_web_httpauth::extractors::bearer::BearerAuth;
use frank_jwt::{decode, encode, Algorithm, Error, ValidationOptions};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct JwtPayload {
    pub level: Level,
    pub exp: i64,
}
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Level {
    Admin,
}

pub fn encode_jwt(hs256_private_key: &str, payload: JwtPayload) -> Result<String, ()> {
    let payload = json!(payload);
    let header = json!({});
    encode(header, &hs256_private_key, &payload, Algorithm::HS256).map_err(|_| ())
}

pub fn decode_jwt(hs256_private_key: &str, jwt: &str) -> Result<JwtPayload, JwtValidationError> {
    let (_header, payload) = decode(
        &jwt,
        &hs256_private_key,
        Algorithm::HS256,
        &ValidationOptions::default(), // default check expiration time
    )
    .map_err(|err| match err {
        Error::SignatureExpired | Error::ExpirationInvalid => JwtValidationError::Expired,
        Error::SignatureInvalid | Error::OpenSslError(_) | Error::ProtocolError(_) => {
            JwtValidationError::Expired
        }
        _ => JwtValidationError::BadFormat,
    })?;

    serde_json::from_value(payload).map_err(|_| JwtValidationError::BadFormat)
}

#[derive(Debug)]
pub enum JwtValidationError {
    BadSignature,
    Expired,
    BadFormat,
}

pub fn valid_jwt_admin(hs256_private_key: &str, jwt: &str) -> Result<(), JwtValidationError> {
    let user = decode_jwt(hs256_private_key, jwt)?;

    match user.level {
        Level::Admin => Ok(()),
        // _ => Err(ApiError("Sorry u cant do that :(".to_owned())),
    }
}
