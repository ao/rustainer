use bollard::volume::ListVolumesOptions;
use bollard::Docker;
use serde::{Deserialize, Serialize};

/// Represents a Docker volume
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Volume {
    /// Volume name
    pub name: String,
    /// Volume driver
    pub driver: String,
    /// Volume mountpoint
    pub mountpoint: String,
    /// Volume creation time
    pub created_at: Option<String>,
    /// Volume labels
    pub labels: Option<std::collections::HashMap<String, String>>,
}

/// List all volumes
pub async fn list_volumes(docker: &Docker) -> anyhow::Result<Vec<Volume>> {
    let options = Some(ListVolumesOptions::<String> {
        ..Default::default()
    });

    let volumes = docker.list_volumes(options).await?;

    let result = volumes
        .volumes
        .unwrap_or_default()
        .into_iter()
        .map(|v| Volume {
            name: v.name,
            driver: v.driver,
            mountpoint: v.mountpoint,
            created_at: v.created_at,
            labels: Some(v.labels),
        })
        .collect();

    Ok(result)
}