use dioxus::prelude::*;

use crate::backend::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};

#[cfg(feature = "server")]
use crate::backend::database::connection::get_db;

#[cfg(feature = "server")]
use crate::backend::domains::users::user_entity::User;

#[cfg(feature = "server")]
use crate::utils::password_hash::{hash_password, verify_password};

#[post("/api/login")]
pub async fn login(
    login_request: LoginRequest,
) -> Result<LoginResponse, HttpError> {
    let db = get_db().await;

    let result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = ?"
    )
    .bind(&login_request.email)
    .fetch_optional(db)
    .await;

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

    if !is_password_valid {
        return HttpError::unauthorized(
            "Invalid username or password",
        );
    }

    Ok(LoginResponse {
        message: "Login successful".to_string(),
    })
}


#[post("/api/register")]
pub async fn register(
    register_request: RegisterRequest,
) -> Result<RegisterResponse, HttpError> {
    let db = get_db().await;

    let password_hash = match hash_password(&register_request.password) {
        Ok(hash) => hash,
        Err(err) => {
            error!("Password hashing failed: {:?}", err);
            return HttpError::internal_server_error(
                "Failed to hash password",
            );
        }
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO users (email, username, password_hash, is_active, is_verified)
        VALUES (?, ?, ?, ?, ?)
        "#,
        register_request.email,
        register_request.username,
        password_hash,
        true,
        true
    )
    .execute(db)
    .await;

    let insert_result = match result {
        Ok(res) => res,
        Err(err) => {
            error!("Register failed: {:?}", err);

            if let sqlx::Error::Database(db_err) = &err {
                let msg = db_err.message();

                if msg.contains("email") {
                    return HttpError::bad_request(
                        "Email already exists",
                    );
                }

                if msg.contains("username") {
                    return HttpError::bad_request(
                        "Username already exists",
                    );
                }
            }

            return HttpError::internal_server_error(
                "Failed to create user",
            );
        }
    };

    // SQLite: get last inserted id
    let user_id = insert_result.last_insert_rowid();

    // Fetch created user
    let user_results = sqlx::query!(
        r#"
        SELECT id, email, username
        FROM users
        WHERE id = ?
        "#,
        user_id
    )
    .fetch_one(db)
    .await;

    let user = match user_results {
        Err(err) => {
            error!("Fetch created user failed: {:?}", err);
            return HttpError::internal_server_error(
                "Failed to fetch created user",
            );
        }
        Ok(user) => user,
    };

    Ok(RegisterResponse {
        message: format!("User {} registered successfully", user.username),
    })
}
