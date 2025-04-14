//! Database module for Rustainer.
//!
//! This module provides functionality for database connections, migrations,
//! and operations.

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;
use std::time::Duration;

/// Initialize the database connection pool.
pub async fn init_db_pool(database_url: &str) -> Result<SqlitePool> {
    // Ensure the database file exists
    if database_url.starts_with("sqlite:") {
        let path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
        if !Path::new(path).exists() {
            // Create parent directories if they don't exist
            if let Some(parent) = Path::new(path).parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            // Create an empty file
            std::fs::File::create(path)?;
        }
    }

    // Create the connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    // Run migrations
    run_migrations(&pool).await?;

    Ok(pool)
}

/// Run database migrations.
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./src/db/migrations")
        .run(pool)
        .await?;

    Ok(())
}

/// Initialize the database with default data.
pub async fn init_default_data(pool: &SqlitePool) -> Result<()> {
    tracing::info!("Initializing default data");
    
    // Check if admin user exists
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = 'admin'")
        .fetch_one(pool)
        .await?;
    
    let admin_exists = row.0 > 0;
    
    if !admin_exists {
        tracing::info!("Creating default admin user");
        
        // Create a salt for password hashing
        let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
        
        // Hash the default password (admin)
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(b"admin", &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();
        
        // Create admin user
        let now = chrono::Utc::now();
        let id = uuid::Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO users (id, username, password_hash, role, email, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind("admin")
        .bind(password_hash)
        .bind("admin")
        .bind("admin@example.com")
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        
        tracing::info!("Default admin user created");
    } else {
        tracing::info!("Admin user already exists");
    }

    Ok(())
}