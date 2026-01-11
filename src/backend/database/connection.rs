#![cfg(feature = "server")]

use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;
use std::path::Path;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

/// Initialize the database connection (call once at server startup)
pub async fn init_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        let db_path = "./data/app.db";

        // Ensure the directory exists
        if let Some(parent) = Path::new(db_path).parent() {
            std::fs::create_dir_all(parent)
                .expect("Failed to create database directory");
        }

        let database_url = format!("sqlite://{}?mode=rwc", db_path);

        Database::connect(&database_url)
            .await
            .expect("Failed to connect to database")
    })
    .await
}

/// Get a reference to the initialized database
/// (safe to call inside handlers / server functions)
pub fn db() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized. Call init_db() first.")
}
