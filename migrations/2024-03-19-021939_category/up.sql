-- Your SQL goes here

CREATE TABLE categories (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(32) NOT NULL,
    url_name VARCHAR(256) NOT NULL UNIQUE,
    description VARCHAR(256) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    published BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO categories (name, url_name, description, published)
VALUES 
('Moda', 'moda', 'Categorias de moda.', TRUE),
('Roupas Masculinas', 'roupas-masculinas', 'Categoria de roupas masculinas.', TRUE),
('Roupas Femininas', 'roupas-femininas', 'Categoria de roupas femininas.', TRUE),
('Acessórios', 'acessorios', 'Categoria de acessórios.', TRUE),
('Bolsas', 'bolsas', 'Categoria de bolsas.', TRUE),
('Jóias', 'joias', 'Categoria de jóias.', TRUE),
('Perfumes', 'perfumes', 'Categorias de perfumes.', TRUE),
('Perfumes Masculinos', 'perfumes-masculinos', 'Categoria de perfumes masculinos.', TRUE),
('Perfumes Femininos', 'perfumes-femininos', 'Categoria de perfumes femininos.', TRUE),
('Perfumes Unissex', 'perfumes-unissex', 'Categoria de perfumes unissex.', TRUE);