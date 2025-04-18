use anyhow::{Context, Result};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tracing::info;

pub async fn init_db_pool(database_url: &str) -> Result<Pool<Sqlite>> {
    // Create the database if it doesn't exist
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        info!("Creating database at {}", database_url);
        Sqlite::create_database(database_url)
            .await
            .context("Failed to create database")?;
    }

    // Create a connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to database")?;

    // Run migrations
    info!("Running database migrations");
    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &Pool<Sqlite>) -> Result<()> {
    // Create users table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to create users table")?;

    // Create applications table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS applications (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            domain TEXT UNIQUE NOT NULL,
            container_id TEXT,
            container_port INTEGER NOT NULL,
            enabled BOOLEAN NOT NULL DEFAULT 1,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to create applications table")?;

    Ok(())
}

pub async fn init_default_data(pool: &Pool<Sqlite>) -> Result<()> {
    // Check if admin user exists
    let admin_exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE username = ?"
    )
    .bind("admin")
    .fetch_one(pool)
    .await
    .context("Failed to check if admin user exists")?;

    // Create admin user if it doesn't exist
    if admin_exists == 0 {
        info!("Creating default admin user");
        
        // Generate password hash (simplified for now)
        let password_hash = format!("admin_password_hash_{}", rand::random::<u64>());

        // Insert admin user
        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, role)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind("admin")
        .bind(password_hash)
        .bind("admin")
        .execute(pool)
        .await
        .context("Failed to create admin user")?;
    }

    Ok(())
}