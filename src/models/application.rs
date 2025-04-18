use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub container_id: Option<String>,
    pub container_port: i64,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Application {
    pub fn new(name: String, domain: String, container_id: Option<String>, container_port: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            domain,
            container_id,
            container_port,
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.updated_at = Utc::now();
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = Utc::now();
    }

    pub fn update_container(&mut self, container_id: Option<String>) {
        self.container_id = container_id;
        self.updated_at = Utc::now();
    }
}