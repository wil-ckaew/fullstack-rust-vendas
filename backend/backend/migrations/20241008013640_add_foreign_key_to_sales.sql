-- Add migration script here
ALTER TABLE sales ADD CONSTRAINT fk_client FOREIGN KEY (client_id) REFERENCES clients(id);
ALTER TABLE sales ADD CONSTRAINT fk_product FOREIGN KEY (product_id) REFERENCES products(id);
