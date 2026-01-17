#![cfg(feature = "server")]

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: i64,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(flatten)]
    pub user: UserClaims,
    pub exp: usize,
}

#[derive(Debug, Clone, Copy)]
enum TokenKind {
    Access,
    Refresh,
}

impl TokenKind {
    fn secret(self) -> Vec<u8> {
        let key = match self {
            TokenKind::Access => "JWT_ACCESS_TOKEN_SECRET",
            TokenKind::Refresh => "JWT_REFRESH_TOKEN_SECRET",
        };

        env::var(key)
            .expect("JWT secret not set")
            .into_bytes()
    }

    fn ttl(self) -> Duration {
        match self {
            TokenKind::Access => Duration::hours(24),
            TokenKind::Refresh => Duration::days(7),
        }
    }
}

fn generate_token(
    kind: TokenKind,
    user: UserClaims,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now() + kind.ttl())
        .timestamp() as usize;

    let claims = Claims {
        user,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&kind.secret()),
    )
}

fn verify_token(
    kind: TokenKind,
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    Ok(decode::<Claims>(
        token,
        &DecodingKey::from_secret(&kind.secret()),
        &Validation::default(),
    )?
    .claims)
}

pub fn generate_access_token(
    user: UserClaims,
) -> Result<String, jsonwebtoken::errors::Error> {
    generate_token(TokenKind::Access, user)
}

pub fn verify_access_token(
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    verify_token(TokenKind::Access, token)
}

pub fn generate_refresh_token(
    user: UserClaims,
) -> Result<String, jsonwebtoken::errors::Error> {
    generate_token(TokenKind::Refresh, user)
}

pub fn verify_refresh_token(
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    verify_token(TokenKind::Refresh, token)
}
