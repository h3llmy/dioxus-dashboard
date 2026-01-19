#![cfg(feature = "server")]

use sqlx::SqlitePool;
use dioxus::prelude::*;

use crate::{
    backend::{
        auth::{
            LoginRequest, LoginResponse,
            RegisterRequest, RegisterResponse,
        }, 
        users::user_repository
    },
    utils::{
        password_hash::{hash_password, verify_password},
        jwt::{UserClaims, generate_access_token, generate_refresh_token},
    },
};

pub async fn login(
    pool: &SqlitePool,
    login_request: LoginRequest,
) -> Result<LoginResponse, HttpError> {
    let result = user_repository::find_by_email(pool, login_request.email.as_str()).await;

    let user = match result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpError::unauthorized(
                "Invalid username or password",
            );
        }
        Err(e) => {
            return HttpError::internal_server_error(
                e.to_string(),
            );
        }
    };

    let is_password_valid =
        verify_password(&user.password_hash, &login_request.password);

    if !&is_password_valid {
        return HttpError::unauthorized(
            "Invalid username or password",
        );
    }

    let access_token: String = match generate_access_token(UserClaims {
        sub: user.id,
        email: user.email.clone(),
    }) {
        Ok(token) => token,
        Err(_) => {
            return HttpError::internal_server_error("failed to generate token");
        }
    };

    let refresh_token: String = match generate_refresh_token(UserClaims {
        sub: user.id,
        email: user.email.clone(),
    }) {
        Ok(token) => token,
        Err(_) => {
            return HttpError::internal_server_error("failed to generate token");
        }
    };

    Ok(LoginResponse {
        message: "Login successful".to_string(),
        access_token,
        refresh_token,
    })
}

pub async fn register(
    pool: &SqlitePool,
    register_request: RegisterRequest,
) -> Result<RegisterResponse, HttpError> {
    // 1️⃣ Hash password (explicit match)
    let password_hash = match hash_password(&register_request.password) {
        Ok(hash) => hash,
        Err(err) => {
            error!("Password hashing failed: {:?}", err);
            return HttpError::internal_server_error("Failed to hash password");
        }
    };

    let result =  user_repository::create_user(pool, &register_request.email, &register_request.username, &password_hash).await;

    let user = match result {
        Ok(user) => user,
        Err(err) => {
            error!("Register failed: {:?}", err);

            if let sqlx::Error::Database(db_err) = &err {
                let msg = db_err.message();

                if msg.contains("email") {
                    return HttpError::bad_request("Email already exists");
                }

                if msg.contains("username") {
                    return HttpError::bad_request("Username already exists");
                }
            }

            return HttpError::internal_server_error("Failed to create user");
        }
    };

    let access_token: String = match generate_access_token(UserClaims {
        sub: user.id,
        email: user.email.clone(),
    }) {
        Ok(token) => token,
        Err(_) => {
            return HttpError::internal_server_error("failed to generate token");
        }
    };

    let refresh_token: String = match generate_refresh_token(UserClaims {
        sub: user.id,
        email: user.email.clone(),
    }) {
        Ok(token) => token,
        Err(_) => {
            return HttpError::internal_server_error("failed to generate token");
        }
    };

    // 3️⃣ Response
    Ok(RegisterResponse {
        message: format!("User {} registered successfully", user.username),
        access_token,
        refresh_token,
    })
}
