-- Add migration script here
CREATE INDEX idx_client_email ON clients(email);
