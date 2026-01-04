use dioxus::prelude::*;
use dioxus::fullstack::ServerFnError;

// Our form endpoint
#[post("/api/login")]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    // Verify the username and password.
    // In a real application, you'd check these against a database.
    if username == "admin" && password == "password" {
        Ok("Login successful".to_string())
    } else {
        Err(ServerFnError::new("Invalid username or password"))
    }
}

