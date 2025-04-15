//! Models for authentication and authorization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;

/// User roles with different permission levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Admin role with full access to all features.
    Admin,
    /// Operator role with access to container operations but not user management.
    Operator,
    /// Viewer role with read-only access.
    Viewer,
}

impl Role {
    /// Convert a string to a Role.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(Role::Admin),
            "operator" => Some(Role::Operator),
            "viewer" => Some(Role::Viewer),
            _ => None,
        }
    }

    /// Convert a Role to a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Operator => "operator",
            Role::Viewer => "viewer",
        }
    }

    /// Check if the role has permission to perform an action.
    pub fn can(&self, action: &str) -> bool {
        match (self, action) {
            // Admin can do everything
            (Role::Admin, _) => true,
            
            // Operator can manage containers but not users
            (Role::Operator, "view_containers") => true,
            (Role::Operator, "manage_containers") => true,
            (Role::Operator, "view_volumes") => true,
            (Role::Operator, "view_networks") => true,
            
            // Viewer can only view resources
            (Role::Viewer, "view_containers") => true,
            (Role::Viewer, "view_volumes") => true,
            (Role::Viewer, "view_networks") => true,
            
            // Default deny
            _ => false,
        }
    }
}

/// User model representing a Rustainer user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for the user.
    pub id: Uuid,
    /// Username for login.
    pub username: String,
    /// Hashed password.
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// User's role.
    pub role: Role,
    /// User's email address.
    pub email: Option<String>,
    /// When the user was created.
    pub created_at: DateTime<Utc>,
    /// When the user was last updated.
    pub updated_at: DateTime<Utc>,
    /// When the user last logged in.
    pub last_login: Option<DateTime<Utc>>,
}

impl FromRow<'_, SqliteRow> for User {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let role_str: String = row.try_get("role")?;
        let role = Role::from_str(&role_str).unwrap_or(Role::Viewer);
        
        // Get the ID as a string and parse it to UUID
        let id_str: String = row.try_get("id")?;
        let id = match Uuid::parse_str(&id_str) {
            Ok(uuid) => uuid,
            Err(e) => {
                tracing::error!("Failed to parse UUID from string '{}': {}", id_str, e);
                return Err(sqlx::Error::ColumnDecode {
                    index: "id".to_string(),
                    source: Box::new(e),
                });
            }
        };
        
        Ok(User {
            id,
            username: row.try_get("username")?,
            password_hash: row.try_get("password_hash")?,
            role,
            email: row.try_get("email")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            last_login: row.try_get("last_login")?,
        })
    }
}

/// Claims for JWT tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Username
    pub username: String,
    /// User role
    pub role: Role,
    /// Issued at timestamp
    pub iat: i64,
    /// Expiration timestamp
    pub exp: i64,
}

/// User creation request.
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    /// Username for the new user
    pub username: String,
    /// Password in plain text (will be hashed)
    pub password: String,
    /// User's role
    pub role: Role,
    /// User's email address
    pub email: Option<String>,
}

/// User login request.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username for login
    pub username: String,
    /// Password in plain text
    pub password: String,
}

/// Authentication response with token.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT token
    pub token: String,
    /// User information
    pub user: UserResponse,
}

/// User response without sensitive information.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// User ID
    pub id: Uuid,
    /// Username
    pub username: String,
    /// User role
    pub role: Role,
    /// User email
    pub email: Option<String>,
    /// When the user was created
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            role: user.role,
            email: user.email,
            created_at: user.created_at,
        }
    }
}