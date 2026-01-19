use dioxus::prelude::*;
use validator::Validate;

use crate::backend::{auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse}, users::User};

#[cfg(feature = "server")]
use crate::backend::{database::connection::get_db, auth::auth_service};

#[post("/api/login")]
pub async fn login(
    login_request: LoginRequest,
) -> Result<LoginResponse, HttpError> {
    let pool = get_db().await;

    auth_service::login(pool, login_request).await
}


#[post("/api/register")]
pub async fn register(
    register_request: RegisterRequest,
) -> Result<RegisterResponse, HttpError> {
    if let Err(err) = register_request.validate() {
        let error_body = serde_json::json!({
            "message": "Validation failed",
            "errors": err.field_errors()
        });

        return HttpError::bad_request(error_body.to_string());
    }
    let db = get_db().await;

    auth_service::register(db, register_request).await
}
