// #![cfg(feature = "server")]

// use sqlx::PgPool;
// use tokio::sync::OnceCell;

// static DB: OnceCell<PgPool> = OnceCell::const_new();

// async fn init_db() -> PgPool {
//     PgPool::connect("postgresql://postgres:postgres@localhost:5432/postgres")
//         .await
//         .expect("Failed to connect to Postgres")
// }

// pub async fn get_db() -> &'static PgPool {
//     DB.get_or_init(init_db).await
// }


#![cfg(feature = "server")]

use sqlx::SqlitePool;
use tokio::sync::OnceCell;
use std::env;

static DB: OnceCell<SqlitePool> = OnceCell::const_new();


async fn init_db() -> SqlitePool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn get_db() -> &'static SqlitePool {
    DB.get_or_init(init_db).await
}
