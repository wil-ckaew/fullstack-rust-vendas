-- Add migration script here
ALTER TABLE products ALTER COLUMN price TYPE DECIMAL(12, 2);
