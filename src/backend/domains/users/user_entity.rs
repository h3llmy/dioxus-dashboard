#![cfg(feature = "server")]

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}