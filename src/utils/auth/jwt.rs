use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use chrono::{Duration, Utc};
use axum::http::StatusCode;
use dotenv::dotenv;
use std::env;

use super::Claims;

pub fn encode_jwt(user_id: String, name: String, username: String, expires_in: i64) -> Result<String, StatusCode> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let now = Utc::now();
    let exp = (now + Duration::try_seconds(expires_in).unwrap()).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claim = Claims { 
        sub: user_id,
        iat, 
        exp, 
        name,
        username
    };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: &str) -> Result<TokenData<Claims>, StatusCode> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode(
        jwt,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
}