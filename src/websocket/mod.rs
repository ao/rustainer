//! WebSocket server for real-time updates.

pub mod docker_events;

pub use docker_events::DockerEventListener;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing::{debug, error, info};
use uuid::Uuid;

/// WebSocket event types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Container events.
    ContainerStart,
    ContainerStop,
    ContainerCreate,
    ContainerDelete,
    ContainerRestart,
    ContainerPause,
    ContainerUnpause,
    /// Image events.
    ImagePull,
    ImageDelete,
    ImageBuild,
    /// Volume events.
    VolumeCreate,
    VolumeDelete,
    /// Network events.
    NetworkCreate,
    NetworkDelete,
    NetworkConnect,
    NetworkDisconnect,
    /// Compose events.
    ComposeUp,
    ComposeDown,
    /// System events.
    SystemInfo,
    /// Connection events.
    Connected,
    Disconnected,
    /// Error events.
    Error,
}

/// WebSocket event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event type.
    pub event_type: EventType,
    /// Event payload.
    pub payload: serde_json::Value,
    /// Event timestamp.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event {
    /// Create a new event.
    pub fn new(event_type: EventType, payload: serde_json::Value) -> Self {
        Self {
            event_type,
            payload,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// WebSocket connection manager.
#[derive(Debug, Clone)]
pub struct WebSocketManager {
    /// Event sender.
    pub tx: broadcast::Sender<Event>,
    /// Connected clients.
    pub clients: Arc<Mutex<HashMap<Uuid, String>>>,
}

impl WebSocketManager {
    /// Create a new WebSocket manager.
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            tx,
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Broadcast an event to all connected clients.
    pub fn broadcast(&self, event: Event) {
        if let Err(e) = self.tx.send(event) {
            error!("Failed to broadcast event: {}", e);
        }
    }

    /// Add a client to the manager.
    pub fn add_client(&self, id: Uuid, user: String) {
        let mut clients = self.clients.lock().unwrap();
        clients.insert(id, user.clone());
        info!("Client connected: {} ({})", id, user);
        info!("Connected clients: {}", clients.len());
    }

    /// Remove a client from the manager.
    pub fn remove_client(&self, id: &Uuid) {
        let mut clients = self.clients.lock().unwrap();
        if let Some(user) = clients.remove(id) {
            info!("Client disconnected: {} ({})", id, user);
        }
        info!("Connected clients: {}", clients.len());
    }
}

/// Handler for WebSocket connections.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, app_state.ws_manager.clone()))
}

// Import the AppState from the app_state module
use crate::app_state::AppState;

/// Handle a WebSocket connection.
async fn handle_socket(socket: WebSocket, manager: WebSocketManager) {
    // Split the socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // Generate a unique ID for this connection
    let id = Uuid::new_v4();

    // Add the client to the manager
    manager.add_client(id, "anonymous".to_string());

    // Subscribe to the broadcast channel
    let mut rx = manager.tx.subscribe();

    // Send a connected event to the client
    let connected_event = Event::new(
        EventType::Connected,
        serde_json::json!({
            "id": id.to_string(),
            "message": "Connected to Rustainer WebSocket server"
        }),
    );

    if let Ok(msg) = serde_json::to_string(&connected_event) {
        if let Err(e) = sender.send(Message::Text(msg)).await {
            error!("Failed to send connected event: {}", e);
        }
    }

    // Spawn a task to forward broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            // Serialize the event to JSON
            if let Ok(msg) = serde_json::to_string(&event) {
                // Send the message to the client
                if let Err(e) = sender.send(Message::Text(msg)).await {
                    error!("Failed to send message: {}", e);
                    break;
                }
            }
        }
    });

    // Handle incoming messages from the client
    let manager_clone = manager.clone();
    let id_clone = id;
    let mut recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    debug!("Received message: {}", text);
                    // Parse the message as JSON
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        // Handle the message
                        handle_client_message(json, &manager_clone).await;
                    }
                }
                Ok(Message::Close(_)) => {
                    debug!("Client closed connection");
                    break;
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    break;
                }
                _ => {}
            }
        }

        // Remove the client when the connection is closed
        manager_clone.remove_client(&id_clone);
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    // Remove the client when the connection is closed
    manager.remove_client(&id);
}

/// Handle a message from a client.
async fn handle_client_message(message: serde_json::Value, manager: &WebSocketManager) {
    // For now, just log the message
    debug!("Client message: {:?}", message);
}
