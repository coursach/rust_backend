use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use lazy_static::lazy_static;
use rocket::{
    http::Status,
    //request::{FromRequest, Outcome},
    response::status::Custom,
};
use serde::{Deserialize, Serialize};

// TODO: this has an extra trailing space to cause the test to fail
// This is to demonstrate shuttle will not deploy when a test fails.
// FIX: remove the extra space character and try deploying again
const BEARER: &str = "Bearer ";

/// Key used for symmetric token encoding
const SECRET: &str = "secret";

lazy_static! {
    /// Time before token expires (aka exp claim)
    static ref TOKEN_EXPIRATION: Duration = Duration::days(180);
}

// Used when decoding a token to `Claims`
#[derive(Debug, PartialEq)]
pub(crate) enum AuthenticationError {
    Missing,
    Decoding(String),
    Expired,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Claims {
    pub(crate) name: String,
    exp: usize,
}


impl Claims {
    pub(crate) fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            exp: 0,
        }
    }

    /// Create a `Claims` from a 'Bearer <token>' value
    pub fn from_authorization(value: &str) -> Result<Self, AuthenticationError> {
        let token = value.strip_prefix(BEARER).map(str::trim);

        if token.is_none() {
            return Err(AuthenticationError::Missing);
        }

        // Safe to unwrap as we just confirmed it is not none
        let token = token.unwrap();

        // Use `jsonwebtoken` to get the claims from a JWT
        // Consult the `jsonwebtoken` documentation for using other algorithms and validations (the default validation just checks the expiration claim)
        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
            .map_err(|e| match e.kind() {
                ErrorKind::ExpiredSignature => AuthenticationError::Expired,
                _ => AuthenticationError::Decoding(e.to_string()),
            })?;

        Ok(token.claims)
    }

    /// Converts this claims into a token string
    pub(crate) fn into_token(mut self) -> Result<String, Custom<String>> {
        let expiration = Utc::now()
            .checked_add_signed(*TOKEN_EXPIRATION)
            .expect("failed to create an expiration time")
            .timestamp();

        self.exp = expiration as usize;

        // Construct and return JWT using `jsonwebtoken`
        // Consult the `jsonwebtoken` documentation for using other algorithms and asymmetric keys
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
            .map_err(|e| Custom(Status::BadRequest, e.to_string()))?;

        Ok(token)
    }
}
