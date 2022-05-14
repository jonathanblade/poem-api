use sqlx::{migrate, pool::PoolOptions, SqlitePool};
use std::time::Duration;
use tokio::process::Command;

use crate::db::repo::UserRepo;
use crate::scheme::InsertUser;
use crate::service::auth_service::AuthService;

async fn create_pool(db_url: &str, db_max_conn: u32, db_conn_timeout: Duration) -> SqlitePool {
    match PoolOptions::new()
        .max_connections(db_max_conn)
        .connect_timeout(db_conn_timeout)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Failed to create database connection pool ({}).", e),
    }
}

async fn run_migrations(pool: &SqlitePool) {
    match migrate!("./migrations").run(pool).await {
        Ok(_) => (),
        Err(e) => panic!("Failed to run database migrations ({}).", e),
    }
}

async fn reset_db(db_url: &str) {
    Command::new("sqlx")
        .env("DATABASE_URL", db_url)
        .args(["database", "drop", "-y"])
        .output()
        .await
        .expect("Failed to drop database.");
    Command::new("sqlx")
        .env("DATABASE_URL", db_url)
        .args(["database", "create"])
        .output()
        .await
        .expect("Failed to create database.");
}

async fn populate_db_for_tests(pool: &SqlitePool) {
    let admin = InsertUser {
        username: "admin".to_string(),
        password_hash: AuthService::hash_password("12345").unwrap(),
        is_superuser: 1,
    };
    let manager = InsertUser {
        username: "manager".to_string(),
        password_hash: AuthService::hash_password("12345").unwrap(),
        is_superuser: 0,
    };
    UserRepo::insert_user(pool, admin).await.unwrap();
    UserRepo::insert_user(pool, manager).await.unwrap();
}

pub async fn prepare_db(db_url: &str, test_mode: bool) -> SqlitePool {
    if test_mode {
        reset_db(db_url).await;
    }
    let pool = create_pool(db_url, 5, Duration::from_secs(5)).await;
    run_migrations(&pool).await;
    if test_mode {
        populate_db_for_tests(&pool).await;
    }
    pool
}
