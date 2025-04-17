-- Create services table
CREATE TABLE IF NOT EXISTS services (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL UNIQUE,
    service_type TEXT NOT NULL,
    target TEXT NOT NULL,
    port INTEGER NOT NULL,
    ssl_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ssl_cert_path TEXT,
    ssl_key_path TEXT,
    ssl_auto_generate BOOLEAN NOT NULL DEFAULT FALSE,
    headers TEXT,  -- JSON serialized headers
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create index on domain for quick lookups
CREATE INDEX IF NOT EXISTS idx_services_domain ON services(domain);