-- Your SQL goes here

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    url_name VARCHAR(512) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    image VARCHAR(256),
    brand_id INTEGER,
    category_id INTEGER,
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    published BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (brand_id) REFERENCES brands(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE INDEX idx_products_name ON products (name);
CREATE INDEX idx_products_published ON products (published);