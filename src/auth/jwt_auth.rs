use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::users::LoginData;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    email: String,
    exp: i64,
}

pub fn get_jwt(user: LoginData) -> Result<String, String> {
    let claim = Claims {
        email: user.email.unwrap(),
        exp: (Utc::now() + Duration::minutes(1)).timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("mysercret".as_bytes()),
    )
    .map_err(|e| e.to_string());
    token
}

pub fn decode_jwt(token: &str) -> Result<LoginData, String> {
    let token_data = decode::<LoginData>(
        &token,
        &DecodingKey::from_secret("mysercret".as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),

        Err(e) => Err(e.to_string()),
    }
}
