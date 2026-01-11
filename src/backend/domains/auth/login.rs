use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    message: String,
}

// Our form endpoint
#[post("/api/login")]
pub async fn login(login_request: LoginRequest) -> Result<LoginResponse, HttpError> {
    // let db = crate::backend::database::connection::db();
    if login_request.username == "admin@some.mail" && login_request.password == "password" {
        Ok(LoginResponse {
            message: "Login successful".to_string(),
        })
    } else {
        HttpError::unauthorized("Invalid username or password".to_string())
    }
}
