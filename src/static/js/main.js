// Main JavaScript file for Rustainer

document.addEventListener('DOMContentLoaded', function() {
    // Initialize theme
    initTheme();
    
    // Add event listeners
    setupEventListeners();
});

// Initialize theme based on cookie or system preference
function initTheme() {
    const theme = getCookie('theme') || 'dark';
    document.documentElement.setAttribute('data-theme', theme);
    updateThemeIcon(theme);
}

// Set up event listeners for interactive elements
function setupEventListeners() {
    // Theme toggle button
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
        themeToggle.addEventListener('click', function() {
            const currentTheme = document.documentElement.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            
            // Update theme icon immediately for better UX
            updateThemeIcon(newTheme);
        });
    }
    
    // Container action buttons
    setupContainerActions();
}

// Update the theme icon based on current theme
function updateThemeIcon(theme) {
    const themeIcon = document.querySelector('.theme-icon');
    if (themeIcon) {
        themeIcon.textContent = theme === 'dark' ? '☀️' : '🌙';
    }
}

// Set up container action buttons
function setupContainerActions() {
    // Add confirmation for destructive actions
    const dangerButtons = document.querySelectorAll('.btn-danger');
    dangerButtons.forEach(button => {
        button.addEventListener('click', function(e) {
            if (!confirm('Are you sure you want to perform this action?')) {
                e.preventDefault();
            }
        });
    });
}

// Helper function to get cookie value by name
function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
}

// Format container status for display
function formatStatus(status) {
    if (!status) return 'unknown';
    
    status = status.toLowerCase();
    
    if (status.includes('running')) return 'running';
    if (status.includes('created')) return 'created';
    if (status.includes('exited')) return 'stopped';
    if (status.includes('paused')) return 'paused';
    
    return status;
}

// Format bytes to human-readable format
function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

// Format timestamp to readable date
function formatTimestamp(timestamp) {
    if (!timestamp) return 'N/A';
    
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
}