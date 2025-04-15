use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::sqlite::SqlitePool;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the database
    let pool = SqlitePool::connect("sqlite:data/rustainer.db").await?;
    
    // Create a salt for password hashing
    let salt = SaltString::generate(&mut OsRng);
    
    // Hash the password (admin)
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(b"admin", &salt)?.to_string();
    
    println!("Generated password hash: {}", password_hash);
    
    // Update the admin user's password
    let result = sqlx::query("UPDATE users SET password_hash = ? WHERE username = ?")
        .bind(&password_hash)
        .bind("admin")
        .execute(&pool)
        .await?;
    
    println!("Updated {} rows", result.rows_affected());
    
    Ok(())
}