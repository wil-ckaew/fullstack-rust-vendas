-- Add migration script here
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    price DOUBLE PRECISION NOT NULL,
    stock_quantity INT NOT NULL
);
