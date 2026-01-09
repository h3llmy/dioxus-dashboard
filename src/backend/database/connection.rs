use sea_orm::{Database, DatabaseConnection};

#[cfg(feature = "server")]
thread_local! {
    pub static DB = Database::connect("postgres://user:pass@host/dbname").await?;
}