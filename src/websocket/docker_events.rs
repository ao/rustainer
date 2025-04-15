//! Docker event listener for real-time updates.

use crate::websocket::{Event, EventType, WebSocketManager};
use bollard::{
    container::ListContainersOptions,
    errors::Error as BollardError,
    system::EventsOptions,
    models::EventMessage as DockerEventMessage,
    image::ListImagesOptions,
    network::ListNetworksOptions,
    volume::ListVolumesOptions,
    Docker,
};
use futures::StreamExt;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

/// Docker event listener.
pub struct DockerEventListener {
    /// WebSocket manager.
    pub manager: WebSocketManager,
    /// Docker client.
    pub docker: Docker,
}

impl DockerEventListener {
    /// Create a new Docker event listener.
    pub fn new(manager: WebSocketManager) -> Result<Self, BollardError> {
        // Connect to the Docker daemon
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { manager, docker })
    }

    /// Start listening for Docker events.
    pub async fn start(&self) -> Result<(), BollardError> {
        info!("Starting Docker event listener");

        // Initial resource state broadcast
        self.broadcast_initial_state().await?;

        // Create event options
        let options = EventsOptions::<String> {
            since: None,
            until: None,
            filters: HashMap::new(),
        };

        // Listen for Docker events
        let mut events = self.docker.events(Some(options));

        // Process events
        while let Some(event) = events.next().await {
            match event {
                Ok(event) => {
                    self.process_event(event).await;
                }
                Err(e) => {
                    error!("Error receiving Docker event: {}", e);
                    // Wait a bit before reconnecting
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }

        Ok(())
    }

    /// Process a Docker event.
    async fn process_event(&self, event: DockerEventMessage) {
        // Map Docker event to our event type
        let event_type = match (&event.typ, &event.action) {
            (Some(typ), Some(action)) => {
                match (typ.to_string().as_str(), action.as_str()) {
                    ("container", "start") => EventType::ContainerStart,
                    ("container", "stop") => EventType::ContainerStop,
                    ("container", "create") => EventType::ContainerCreate,
                    ("container", "destroy") => EventType::ContainerDelete,
                    ("container", "restart") => EventType::ContainerRestart,
                    ("container", "pause") => EventType::ContainerPause,
                    ("container", "unpause") => EventType::ContainerUnpause,
                    ("image", "pull") => EventType::ImagePull,
                    ("image", "delete") => EventType::ImageDelete,
                    ("image", "build") => EventType::ImageBuild,
                    ("volume", "create") => EventType::VolumeCreate,
                    ("volume", "destroy") => EventType::VolumeDelete,
                    ("network", "create") => EventType::NetworkCreate,
                    ("network", "destroy") => EventType::NetworkDelete,
                    ("network", "connect") => EventType::NetworkConnect,
                    ("network", "disconnect") => EventType::NetworkDisconnect,
                    _ => {
                        // Unknown event type, just log it
                        info!("Unknown Docker event: {:?}", event);
                        return;
                    }
                }
            },
            _ => {
                // Missing type or action, just log it
                info!("Incomplete Docker event: {:?}", event);
                return;
            }
        };

        // Create payload
        let payload = json!({
            "id": event.actor.as_ref().and_then(|actor| actor.id.clone()),
            "attributes": event.actor.as_ref().and_then(|actor| Some(actor.attributes.clone())),
            "time": event.time,
            "timeNano": event.time_nano,
        });

        // Create and broadcast event
        let ws_event = Event::new(event_type.clone(), payload);
        self.manager.broadcast(ws_event);

        // For certain events, we need to broadcast the updated resource state
        match event_type {
            EventType::ContainerStart | EventType::ContainerStop | EventType::ContainerCreate | EventType::ContainerDelete => {
                self.broadcast_containers().await.ok();
            }
            EventType::ImagePull | EventType::ImageDelete | EventType::ImageBuild => {
                self.broadcast_images().await.ok();
            }
            EventType::VolumeCreate | EventType::VolumeDelete => {
                self.broadcast_volumes().await.ok();
            }
            EventType::NetworkCreate | EventType::NetworkDelete => {
                self.broadcast_networks().await.ok();
            }
            _ => {}
        }
    }

    /// Broadcast initial state of all resources.
    async fn broadcast_initial_state(&self) -> Result<(), BollardError> {
        // Broadcast system info
        self.broadcast_system_info().await?;

        // Broadcast containers
        self.broadcast_containers().await?;

        // Broadcast images
        self.broadcast_images().await?;

        // Broadcast volumes
        self.broadcast_volumes().await?;

        // Broadcast networks
        self.broadcast_networks().await?;

        Ok(())
    }

    /// Broadcast system info.
    async fn broadcast_system_info(&self) -> Result<(), BollardError> {
        let info = self.docker.info().await?;
        let payload = json!(info);
        let event = Event::new(EventType::SystemInfo, payload);
        self.manager.broadcast(event);
        Ok(())
    }

    /// Broadcast containers.
    async fn broadcast_containers(&self) -> Result<(), BollardError> {
        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };
        let containers = self.docker.list_containers(Some(options)).await?;
        let payload = json!({
            "containers": containers,
        });
        let event = Event::new(EventType::ContainerStart, payload);
        self.manager.broadcast(event);
        Ok(())
    }

    /// Broadcast images.
    async fn broadcast_images(&self) -> Result<(), BollardError> {
        let options = ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        };
        let images = self.docker.list_images(Some(options)).await?;
        let payload = json!({
            "images": images,
        });
        let event = Event::new(EventType::ImagePull, payload);
        self.manager.broadcast(event);
        Ok(())
    }

    /// Broadcast volumes.
    async fn broadcast_volumes(&self) -> Result<(), BollardError> {
        let options = ListVolumesOptions::<String> {
            ..Default::default()
        };
        let volumes = self.docker.list_volumes(Some(options)).await?;
        let payload = json!({
            "volumes": volumes,
        });
        let event = Event::new(EventType::VolumeCreate, payload);
        self.manager.broadcast(event);
        Ok(())
    }

    /// Broadcast networks.
    async fn broadcast_networks(&self) -> Result<(), BollardError> {
        let options = ListNetworksOptions::<String> {
            ..Default::default()
        };
        let networks = self.docker.list_networks(Some(options)).await?;
        let payload = json!({
            "networks": networks,
        });
        let event = Event::new(EventType::NetworkCreate, payload);
        self.manager.broadcast(event);
        Ok(())
    }
}