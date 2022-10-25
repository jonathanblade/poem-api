pub mod repo;

use sqlx::{migrate, migrate::MigrateDatabase, pool::PoolOptions, Sqlite, SqlitePool};
use std::time::Duration;

pub async fn prepare_db(db_url: &str) -> SqlitePool {
    if !Sqlite::database_exists(db_url)
        .await
        .expect("Failed to check database exists.")
    {
        Sqlite::create_database(db_url)
            .await
            .expect("Failed to create database.");
    }
    let pool = PoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(db_url)
        .await
        .expect("Failed to create database connection pool.");
    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations.");
    pool
}
