/**
 * WebSocket client for Rustainer.
 * This module handles WebSocket connections to the server for real-time updates.
 */

class RustainerWebSocket {
    /**
     * Create a new WebSocket client.
     */
    constructor() {
        this.socket = null;
        this.connected = false;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 1000; // Start with 1 second delay
        this.eventHandlers = {};
        this.connectionId = null;
    }

    /**
     * Connect to the WebSocket server.
     * @returns {Promise<void>} A promise that resolves when connected.
     */
    async connect() {
        return new Promise((resolve, reject) => {
            try {
                // Determine the WebSocket URL based on the current location
                const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
                const host = window.location.host;
                const wsUrl = `${protocol}//${host}/ws`;

                console.log(`Connecting to WebSocket server at ${wsUrl}`);
                this.socket = new WebSocket(wsUrl);

                // Set up event handlers
                this.socket.onopen = () => {
                    console.log('WebSocket connection established');
                    this.connected = true;
                    this.reconnectAttempts = 0;
                    this.reconnectDelay = 1000;
                    resolve();
                };

                this.socket.onmessage = (event) => {
                    try {
                        const data = JSON.parse(event.data);
                        this.handleMessage(data);
                    } catch (error) {
                        console.error('Error parsing WebSocket message:', error);
                    }
                };

                this.socket.onclose = () => {
                    console.log('WebSocket connection closed');
                    this.connected = false;
                    this.reconnect();
                };

                this.socket.onerror = (error) => {
                    console.error('WebSocket error:', error);
                    reject(error);
                };
            } catch (error) {
                console.error('Error connecting to WebSocket server:', error);
                reject(error);
            }
        });
    }

    /**
     * Reconnect to the WebSocket server.
     */
    reconnect() {
        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.error('Maximum reconnect attempts reached');
            return;
        }

        this.reconnectAttempts++;
        const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
        console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`);

        setTimeout(() => {
            console.log('Attempting to reconnect...');
            this.connect().catch(() => {
                // If reconnect fails, the onclose handler will trigger another reconnect
            });
        }, delay);
    }

    /**
     * Close the WebSocket connection.
     */
    close() {
        if (this.socket) {
            this.socket.close();
        }
    }

    /**
     * Send a message to the WebSocket server.
     * @param {Object} message - The message to send.
     */
    send(message) {
        if (!this.connected) {
            console.error('Cannot send message: WebSocket not connected');
            return;
        }

        try {
            this.socket.send(JSON.stringify(message));
        } catch (error) {
            console.error('Error sending WebSocket message:', error);
        }
    }

    /**
     * Handle a message from the WebSocket server.
     * @param {Object} message - The message received.
     */
    handleMessage(message) {
        console.log('Received WebSocket message:', message);

        // Handle connection event
        if (message.event_type === 'connected') {
            this.connectionId = message.payload.id;
            console.log(`Connected with ID: ${this.connectionId}`);
        }

        // Trigger event handlers
        const eventType = message.event_type;
        if (this.eventHandlers[eventType]) {
            this.eventHandlers[eventType].forEach(handler => {
                try {
                    handler(message.payload);
                } catch (error) {
                    console.error(`Error in event handler for ${eventType}:`, error);
                }
            });
        }

        // Trigger 'all' event handlers
        if (this.eventHandlers['all']) {
            this.eventHandlers['all'].forEach(handler => {
                try {
                    handler(message);
                } catch (error) {
                    console.error('Error in "all" event handler:', error);
                }
            });
        }
    }

    /**
     * Register an event handler.
     * @param {string} eventType - The event type to listen for.
     * @param {Function} handler - The handler function.
     */
    on(eventType, handler) {
        if (!this.eventHandlers[eventType]) {
            this.eventHandlers[eventType] = [];
        }
        this.eventHandlers[eventType].push(handler);
    }

    /**
     * Remove an event handler.
     * @param {string} eventType - The event type.
     * @param {Function} handler - The handler function to remove.
     */
    off(eventType, handler) {
        if (!this.eventHandlers[eventType]) {
            return;
        }
        this.eventHandlers[eventType] = this.eventHandlers[eventType].filter(h => h !== handler);
    }
}

// Create a singleton instance
const rustainerWs = new RustainerWebSocket();

// Export the singleton
window.rustainerWs = rustainerWs;

// Connect when the page loads
document.addEventListener('DOMContentLoaded', () => {
    rustainerWs.connect().catch(error => {
        console.error('Failed to connect to WebSocket server:', error);
    });

    // Set up event handlers for UI updates
    setupEventHandlers();
});

/**
 * Set up event handlers for UI updates.
 */
function setupEventHandlers() {
    // Container events
    rustainerWs.on('container_start', updateContainers);
    rustainerWs.on('container_stop', updateContainers);
    rustainerWs.on('container_create', updateContainers);
    rustainerWs.on('container_delete', updateContainers);
    rustainerWs.on('container_restart', updateContainers);
    
    // Image events
    rustainerWs.on('image_pull', updateImages);
    rustainerWs.on('image_delete', updateImages);
    rustainerWs.on('image_build', updateImages);
    
    // Volume events
    rustainerWs.on('volume_create', updateVolumes);
    rustainerWs.on('volume_delete', updateVolumes);
    
    // Network events
    rustainerWs.on('network_create', updateNetworks);
    rustainerWs.on('network_delete', updateNetworks);
    
    // Compose events
    rustainerWs.on('compose_up', updateCompose);
    rustainerWs.on('compose_down', updateCompose);
    
    // System events
    rustainerWs.on('system_info', updateSystemInfo);
}

/**
 * Update containers in the UI.
 * @param {Object} data - Container data.
 */
function updateContainers(data) {
    console.log('Updating containers:', data);
    // TODO: Implement UI update logic
}

/**
 * Update images in the UI.
 * @param {Object} data - Image data.
 */
function updateImages(data) {
    console.log('Updating images:', data);
    // TODO: Implement UI update logic
}

/**
 * Update volumes in the UI.
 * @param {Object} data - Volume data.
 */
function updateVolumes(data) {
    console.log('Updating volumes:', data);
    // TODO: Implement UI update logic
}

/**
 * Update networks in the UI.
 * @param {Object} data - Network data.
 */
function updateNetworks(data) {
    console.log('Updating networks:', data);
    // TODO: Implement UI update logic
}

/**
 * Update compose stacks in the UI.
 * @param {Object} data - Compose data.
 */
function updateCompose(data) {
    console.log('Updating compose stacks:', data);
    // TODO: Implement UI update logic
}

/**
 * Update system info in the UI.
 * @param {Object} data - System info data.
 */
function updateSystemInfo(data) {
    console.log('Updating system info:', data);
    // TODO: Implement UI update logic
}