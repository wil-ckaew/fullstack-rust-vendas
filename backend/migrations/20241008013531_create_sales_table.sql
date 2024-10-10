-- Add migration script here
CREATE TABLE sales (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id UUID REFERENCES clients(id),
    product_id UUID REFERENCES products(id),
    quantity INT NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    sale_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
