-- Reset admin password to 'admin'
-- This is a simple plaintext password for testing purposes
UPDATE users SET password_hash = 'admin' WHERE username = 'admin';