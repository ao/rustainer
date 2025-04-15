use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::{sqlite::SqlitePool, Row};
use std::error::Error;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Resetting admin password to 'admin'...");
    
    // Connect to the database
    let pool = SqlitePool::connect("sqlite:data/rustainer.db").await?;
    
    // Option 1: Set a plaintext password (for testing only)
    println!("Option 1: Setting plaintext password (for testing only)");
    let result1 = sqlx::query("UPDATE users SET password_hash = ? WHERE username = ?")
        .bind("admin")
        .bind("admin")
        .execute(&pool)
        .await?;
    
    println!("Updated {} rows with plaintext password", result1.rows_affected());
    
    // Option 2: Set a properly hashed password
    println!("Option 2: Setting properly hashed password");
    
    // Create a salt for password hashing
    let salt = SaltString::generate(&mut OsRng);
    
    // Hash the password (admin)
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(b"admin", &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();
    
    println!("Generated password hash: {}", password_hash);
    
    let result2 = sqlx::query("UPDATE users SET password_hash = ? WHERE username = ?")
        .bind(&password_hash)
        .bind("admin")
        .execute(&pool)
        .await?;
    
    println!("Updated {} rows with hashed password", result2.rows_affected());
    
    // Verify the user exists
    let user = sqlx::query("SELECT username, password_hash FROM users WHERE username = 'admin'")
        .fetch_one(&pool)
        .await?;
    
    let username: String = user.try_get("username")?;
    let password_hash: String = user.try_get("password_hash")?;
    
    println!("User '{}' has password_hash: {}", username, password_hash);
    
    println!("Password reset complete!");
    println!("You can now log in with username 'admin' and password 'admin'");
    
    Ok(())
}