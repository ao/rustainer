//! Service management functionality for Rustainer.
//!
//! This module provides functionality for managing services for domain-based routing.

use anyhow::{Result, Context};
use sqlx::{Pool, Sqlite, Row};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::Utc;
use serde_json;

use crate::models::service::{
    Service, ServiceType, SSLConfig,
    CreateServiceRequest, UpdateServiceRequest, ServiceResponse
};

/// Create a new service
pub async fn create_service(
    db: &Pool<Sqlite>,
    request: CreateServiceRequest
) -> Result<Service> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    
    // Convert headers to JSON string if provided
    let headers_json = if let Some(headers) = &request.headers {
        Some(serde_json::to_string(headers)?)
    } else {
        None
    };
    
    // Default SSL config if not provided
    let ssl = request.ssl.unwrap_or(SSLConfig {
        enabled: false,
        cert_path: None,
        key_path: None,
        auto_generate: false,
    });
    
    // Insert into database
    sqlx::query(
        r#"
        INSERT INTO services (
            id, name, domain, service_type, target, port,
            ssl_enabled, ssl_cert_path, ssl_key_path, ssl_auto_generate,
            headers, enabled, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(id.to_string())
    .bind(&request.name)
    .bind(&request.domain)
    .bind(serde_json::to_string(&request.service_type)?)
    .bind(&request.target)
    .bind(request.port as i64)
    .bind(ssl.enabled)
    .bind(ssl.cert_path.as_deref())
    .bind(ssl.key_path.as_deref())
    .bind(ssl.auto_generate)
    .bind(headers_json)
    .bind(true)
    .bind(now)
    .bind(now)
    .execute(db)
    .await
    .context("Failed to insert service into database")?;
    
    // Return the created service
    let service = Service {
        id,
        name: request.name,
        domain: request.domain,
        service_type: request.service_type,
        target: request.target,
        port: request.port,
        ssl,
        headers: request.headers.unwrap_or_default(),
        enabled: true,
        created_at: now,
        updated_at: now,
    };
    
    Ok(service)
}

/// Get a service by ID
pub async fn get_service(
    db: &Pool<Sqlite>,
    id: &Uuid
) -> Result<Option<Service>> {
    let record = sqlx::query(
        r#"
        SELECT
            id, name, domain, service_type, target, port,
            ssl_enabled, ssl_cert_path, ssl_key_path, ssl_auto_generate,
            headers, enabled, created_at, updated_at
        FROM services
        WHERE id = ?
        "#
    )
    .bind(id.to_string())
    .fetch_optional(db)
    .await
    .context("Failed to fetch service from database")?;
    
    match record {
        Some(row) => {
            let id: String = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let domain: String = row.try_get("domain")?;
            let service_type_str: String = row.try_get("service_type")?;
            let target: String = row.try_get("target")?;
            let port: i64 = row.try_get("port")?;
            let ssl_enabled: bool = row.try_get("ssl_enabled")?;
            let ssl_cert_path: Option<String> = row.try_get("ssl_cert_path")?;
            let ssl_key_path: Option<String> = row.try_get("ssl_key_path")?;
            let ssl_auto_generate: bool = row.try_get("ssl_auto_generate")?;
            let headers_json: Option<String> = row.try_get("headers")?;
            let enabled: bool = row.try_get("enabled")?;
            let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at")?;
            let updated_at: chrono::DateTime<chrono::Utc> = row.try_get("updated_at")?;
            
            let service_type: ServiceType = serde_json::from_str(&service_type_str)
                .context("Failed to deserialize service type")?;
            
            let headers: HashMap<String, String> = if let Some(json) = headers_json {
                serde_json::from_str(&json)
                    .context("Failed to deserialize headers")?
            } else {
                HashMap::new()
            };
            
            let ssl = SSLConfig {
                enabled: ssl_enabled,
                cert_path: ssl_cert_path,
                key_path: ssl_key_path,
                auto_generate: ssl_auto_generate,
            };
            
            Ok(Some(Service {
                id: Uuid::parse_str(&id)?,
                name,
                domain,
                service_type,
                target,
                port: port as u16,
                ssl,
                headers,
                enabled,
                created_at,
                updated_at,
            }))
        },
        None => Ok(None),
    }
}

/// List all services
pub async fn list_services(
    db: &Pool<Sqlite>
) -> Result<Vec<Service>> {
    let rows = sqlx::query(
        r#"
        SELECT
            id, name, domain, service_type, target, port,
            ssl_enabled, ssl_cert_path, ssl_key_path, ssl_auto_generate,
            headers, enabled, created_at, updated_at
        FROM services
        ORDER BY name
        "#
    )
    .fetch_all(db)
    .await
    .context("Failed to fetch services from database")?;
    
    let mut services = Vec::with_capacity(rows.len());
    
    for row in rows {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let domain: String = row.try_get("domain")?;
        let service_type_str: String = row.try_get("service_type")?;
        let target: String = row.try_get("target")?;
        let port: i64 = row.try_get("port")?;
        let ssl_enabled: bool = row.try_get("ssl_enabled")?;
        let ssl_cert_path: Option<String> = row.try_get("ssl_cert_path")?;
        let ssl_key_path: Option<String> = row.try_get("ssl_key_path")?;
        let ssl_auto_generate: bool = row.try_get("ssl_auto_generate")?;
        let headers_json: Option<String> = row.try_get("headers")?;
        let enabled: bool = row.try_get("enabled")?;
        let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at")?;
        let updated_at: chrono::DateTime<chrono::Utc> = row.try_get("updated_at")?;
        
        let service_type: ServiceType = serde_json::from_str(&service_type_str)
            .context("Failed to deserialize service type")?;
        
        let headers: HashMap<String, String> = if let Some(json) = headers_json {
            serde_json::from_str(&json)
                .context("Failed to deserialize headers")?
        } else {
            HashMap::new()
        };
        
        let ssl = SSLConfig {
            enabled: ssl_enabled,
            cert_path: ssl_cert_path,
            key_path: ssl_key_path,
            auto_generate: ssl_auto_generate,
        };
        
        services.push(Service {
            id: Uuid::parse_str(&id)?,
            name,
            domain,
            service_type,
            target,
            port: port as u16,
            ssl,
            headers,
            enabled,
            created_at,
            updated_at,
        });
    }
    
    Ok(services)
}

/// Update a service
pub async fn update_service(
    db: &Pool<Sqlite>,
    id: &Uuid,
    request: UpdateServiceRequest
) -> Result<Option<Service>> {
    // First check if the service exists
    let existing = get_service(db, id).await?;
    if existing.is_none() {
        return Ok(None);
    }
    
    let existing = existing.unwrap();
    let now = Utc::now();
    
    // Build the update query dynamically based on what fields are provided
    let mut query = String::from("UPDATE services SET updated_at = ? ");
    let mut params: Vec<String> = vec![now.to_string()];
    
    if let Some(name) = &request.name {
        query.push_str(", name = ? ");
        params.push(name.clone());
    }
    
    if let Some(domain) = &request.domain {
        query.push_str(", domain = ? ");
        params.push(domain.clone());
    }
    
    if let Some(service_type) = &request.service_type {
        query.push_str(", service_type = ? ");
        params.push(serde_json::to_string(service_type)?);
    }
    
    if let Some(target) = &request.target {
        query.push_str(", target = ? ");
        params.push(target.clone());
    }
    
    if let Some(port) = request.port {
        query.push_str(", port = ? ");
        params.push(port.to_string());
    }
    
    if let Some(ssl) = &request.ssl {
        query.push_str(", ssl_enabled = ?, ssl_cert_path = ?, ssl_key_path = ?, ssl_auto_generate = ? ");
        params.push(ssl.enabled.to_string());
        params.push(ssl.cert_path.clone().unwrap_or_default());
        params.push(ssl.key_path.clone().unwrap_or_default());
        params.push(ssl.auto_generate.to_string());
    }
    
    if let Some(headers) = &request.headers {
        query.push_str(", headers = ? ");
        params.push(serde_json::to_string(headers)?);
    }
    
    if let Some(enabled) = request.enabled {
        query.push_str(", enabled = ? ");
        params.push(enabled.to_string());
    }
    
    query.push_str("WHERE id = ?");
    params.push(id.to_string());
    
    // Execute the update query
    let mut db_query = sqlx::query(&query);
    for param in params {
        db_query = db_query.bind(param);
    }
    
    db_query
        .execute(db)
        .await
        .context("Failed to update service in database")?;
    
    // Return the updated service
    get_service(db, id).await
}

/// Delete a service
pub async fn delete_service(
    db: &Pool<Sqlite>,
    id: &Uuid
) -> Result<bool> {
    let result = sqlx::query("DELETE FROM services WHERE id = ?")
        .bind(id.to_string())
        .execute(db)
        .await
        .context("Failed to delete service from database")?;
    
    Ok(result.rows_affected() > 0)
}

/// Get a service by domain
pub async fn get_service_by_domain(
    db: &Pool<Sqlite>,
    domain: &str
) -> Result<Option<Service>> {
    let row = sqlx::query(
        r#"
        SELECT
            id, name, domain, service_type, target, port,
            ssl_enabled, ssl_cert_path, ssl_key_path, ssl_auto_generate,
            headers, enabled, created_at, updated_at
        FROM services
        WHERE domain = ? AND enabled = TRUE
        "#
    )
    .bind(domain)
    .fetch_optional(db)
    .await
    .context("Failed to fetch service from database")?;
    
    match row {
        Some(row) => {
            let id: String = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let domain: String = row.try_get("domain")?;
            let service_type_str: String = row.try_get("service_type")?;
            let target: String = row.try_get("target")?;
            let port: i64 = row.try_get("port")?;
            let ssl_enabled: bool = row.try_get("ssl_enabled")?;
            let ssl_cert_path: Option<String> = row.try_get("ssl_cert_path")?;
            let ssl_key_path: Option<String> = row.try_get("ssl_key_path")?;
            let ssl_auto_generate: bool = row.try_get("ssl_auto_generate")?;
            let headers_json: Option<String> = row.try_get("headers")?;
            let enabled: bool = row.try_get("enabled")?;
            let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at")?;
            let updated_at: chrono::DateTime<chrono::Utc> = row.try_get("updated_at")?;
            
            let service_type: ServiceType = serde_json::from_str(&service_type_str)
                .context("Failed to deserialize service type")?;
            
            let headers: HashMap<String, String> = if let Some(json) = headers_json {
                serde_json::from_str(&json)
                    .context("Failed to deserialize headers")?
            } else {
                HashMap::new()
            };
            
            let ssl = SSLConfig {
                enabled: ssl_enabled,
                cert_path: ssl_cert_path,
                key_path: ssl_key_path,
                auto_generate: ssl_auto_generate,
            };
            
            Ok(Some(Service {
                id: Uuid::parse_str(&id)?,
                name,
                domain,
                service_type,
                target,
                port: port as u16,
                ssl,
                headers,
                enabled,
                created_at,
                updated_at,
            }))
        },
        None => Ok(None),
    }
}

/// Enable a service
pub async fn enable_service(
    db: &Pool<Sqlite>,
    id: &Uuid
) -> Result<Option<Service>> {
    let result = sqlx::query(
        "UPDATE services SET enabled = TRUE, updated_at = ? WHERE id = ?"
    )
    .bind(Utc::now())
    .bind(id.to_string())
    .execute(db)
    .await
    .context("Failed to enable service in database")?;
    
    if result.rows_affected() > 0 {
        get_service(db, id).await
    } else {
        Ok(None)
    }
}

/// Disable a service
pub async fn disable_service(
    db: &Pool<Sqlite>,
    id: &Uuid
) -> Result<Option<Service>> {
    let result = sqlx::query(
        "UPDATE services SET enabled = FALSE, updated_at = ? WHERE id = ?"
    )
    .bind(Utc::now())
    .bind(id.to_string())
    .execute(db)
    .await
    .context("Failed to disable service in database")?;
    
    if result.rows_affected() > 0 {
        get_service(db, id).await
    } else {
        Ok(None)
    }
}