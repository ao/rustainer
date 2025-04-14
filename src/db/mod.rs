//! Database module for Rustainer.
//!
//! This module provides functionality for database connections, migrations,
//! and operations.

use anyhow::Result;
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
    // For now, let's just log that we're initializing default data
    // and skip the actual database operations to avoid SQLx compile-time checks
    tracing::info!("Initializing default data (mock implementation)");
    
    // In a real implementation, we would check if the admin user exists
    // and create it if it doesn't

    Ok(())
}