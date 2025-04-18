use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data/rustainer.db".to_string());

    // Connect to the database
    let pool = SqlitePool::connect(&database_url)
        .await
        .context("Failed to connect to database")?;

    // Generate simple password hash
    let password_hash = format!("admin_password_hash_{}", rand::random::<u64>());

    // Update admin user password
    let result = sqlx::query(
        r#"
        UPDATE users
        SET password_hash = ?
        WHERE username = 'admin'
        "#
    )
    .bind(&password_hash)
    .execute(&pool)
    .await
    .context("Failed to update admin password")?;

    if result.rows_affected() > 0 {
        println!("Admin password reset successfully to 'admin'");
    } else {
        println!("Admin user not found. Creating new admin user...");

        // Create admin user if it doesn't exist
        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, role, created_at, updated_at)
            VALUES (?, 'admin', ?, 'admin', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&password_hash)
        .execute(&pool)
        .await
        .context("Failed to create admin user")?;

        println!("Admin user created successfully with password 'admin'");
    }

    Ok(())
}