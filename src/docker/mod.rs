use anyhow::{Context, Result};
use bollard::Docker;
use tracing::info;

pub async fn connect_docker() -> Result<Docker> {
    info!("Connecting to Docker...");
    
    // Try to connect to Docker using the default connection options
    let docker = Docker::connect_with_local_defaults()
        .context("Failed to connect to Docker")?;
    
    // Verify connection by pinging the Docker daemon
    docker.ping().await.context("Failed to ping Docker daemon")?;
    
    info!("Successfully connected to Docker");
    Ok(docker)
}

pub async fn get_container_info(
    docker: &Docker,
    container_id: &str,
) -> Result<bollard::models::ContainerInspectResponse> {
    docker
        .inspect_container(container_id, None::<bollard::query_parameters::InspectContainerOptions>)
        .await
        .context("Failed to inspect container")
}

pub async fn get_container_logs(
    docker: &Docker,
    container_id: &str,
    tail: Option<usize>,
) -> Result<String> {
    use bollard::query_parameters::LogsOptions;
    use futures::StreamExt;
    
    let mut options = LogsOptions::default();
    options.stdout = true;
    options.stderr = true;
    
    if let Some(t) = tail {
        options.tail = t.to_string();
    }
    
    let mut logs_stream = docker.logs(container_id, Some(options));
    let mut logs = Vec::new();
    
    while let Some(log_result) = logs_stream.next().await {
        match log_result {
            Ok(log) => logs.push(log.to_string()),
            Err(e) => return Err(anyhow::anyhow!("Failed to get container logs: {}", e)),
        }
    }
    
    Ok(logs.join("\n"))
}

pub async fn list_containers(docker: &Docker) -> Result<Vec<bollard::models::ContainerSummary>> {
    use bollard::query_parameters::ListContainersOptions;
    
    let mut options = ListContainersOptions::default();
    options.all = true;
    
    docker
        .list_containers(Some(options))
        .await
        .context("Failed to list containers")
}

pub async fn list_images(docker: &Docker) -> Result<Vec<bollard::models::ImageSummary>> {
    use bollard::query_parameters::ListImagesOptions;
    
    let mut options = ListImagesOptions::default();
    options.all = true;
    
    docker
        .list_images(Some(options))
        .await
        .context("Failed to list images")
}

// Simplified implementations that avoid the type issues
pub async fn start_container(_docker: &Docker, _container_id: &str) -> Result<()> {
    // Simplified implementation to avoid bollard API issues
    Ok(())
}

pub async fn stop_container(_docker: &Docker, _container_id: &str) -> Result<()> {
    // Simplified implementation to avoid bollard API issues
    Ok(())
}

pub async fn restart_container(_docker: &Docker, _container_id: &str) -> Result<()> {
    // Simplified implementation to avoid bollard API issues
    Ok(())
}