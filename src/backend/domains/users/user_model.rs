#![cfg(feature = "server")]

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl User {
    pub fn new(id: i32, email: String, username: String, password_hash: String, is_active: bool, is_verified: bool, created_at: Option<String>, updated_at: Option<String>) -> Self {
        Self { id, email, username, password_hash, is_active, is_verified, created_at, updated_at }
    }
}