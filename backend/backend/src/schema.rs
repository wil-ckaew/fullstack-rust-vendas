pub const CREATE_CLIENTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS clients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    phone VARCHAR NOT NULL
);
"#;

pub const CREATE_PRODUCTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    quantity INTEGER NOT NULL
);
"#;

pub const CREATE_SALES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS sales (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id UUID REFERENCES clients(id) ON DELETE CASCADE,
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    total_price NUMERIC(10, 2) NOT NULL
);
"#;

// Você pode adicionar mais comandos SQL aqui para criar outros relacionamentos ou tabelas adicionais
