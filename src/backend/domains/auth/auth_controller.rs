use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::backend::database::connection::get_db;

#[cfg(feature = "server")]
use crate::backend::domains::users::user_model::User;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
}

#[post("/api/login")]
pub async fn login(
    login_request: LoginRequest,
) -> Result<LoginResponse, HttpError> {
    let db = get_db().await;

    // let create_result = sqlx::query!("INSERT INTO users (email, username, password_hash, is_active, is_verified) VALUES (?, ?, ?, ?, ?)",
    //     login_request.username, login_request.username, login_request.password, true, true)
    //     .execute(db)
    //     .await;

    // if create_result.is_err() {
    //     return HttpError::internal_server_error(
    //         "Failed to create user",
    //     );
    // }

    let result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = ?"
    )
    .bind(&login_request.username)
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

    if user.password_hash != login_request.password {
        return HttpError::unauthorized(
            "Invalid username or password",
        );
    }

    Ok(LoginResponse {
        message: "Login successful".to_string(),
    })
}
