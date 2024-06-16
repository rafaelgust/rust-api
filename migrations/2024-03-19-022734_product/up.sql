-- Your SQL goes here

CREATE TABLE products (
    id UUID PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    url_name VARCHAR(512) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    image VARCHAR(256),
    brand_id INTEGER,
    category_id INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    published BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (brand_id) REFERENCES brands(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE INDEX idx_products_name ON products (name);
CREATE INDEX idx_products_published ON products (published);

-- Vestido de Festa Azul
INSERT INTO products (id, name, url_name, description, image, brand_id, category_id, published)
VALUES ('018e15ba-ff46-7023-545b-bffb6d3515e4', 'Vestido de Festa Azul', 'vestido-festa-azul', 'Vestido elegante para ocasiões especiais, cor azul vibrante e design moderno.', 'vestido_azul.jpg', 1, 3, TRUE);

-- Sapato Social Masculino
INSERT INTO products (id, name, url_name, description, image, brand_id, category_id, published)
VALUES ('018e15ba-ff46-7023-546b-bffb6d3516e4', 'Sapato Social Masculino', 'sapato-social-masculino', 'Sapato de couro genuíno para homens, ideal para eventos formais.', 'sapato_social.jpg', 2, 2, TRUE);

-- Bolsa Feminina de Couro
INSERT INTO products (id, name, url_name, description, image, brand_id, category_id, published)
VALUES ('018e15ba-ff46-7023-547b-bffb6d3517e4', 'Bolsa Feminina de Couro', 'bolsa-feminina-couro', 'Bolsa elegante de couro para mulheres, espaçosa e sofisticada.', 'bolsa_couro.jpg', 3, 5, TRUE);

-- Perfume Masculino Intenso
INSERT INTO products (id, name, url_name, description, image, brand_id, category_id, published)
VALUES ('018e15ba-ff46-7023-548b-bffb6d3518e4', 'Perfume Masculino Intenso', 'perfume-masculino-intenso', 'Fragrância masculina intensa com notas amadeiradas e cítricas.', 'perfume_masculino.jpg', 4, 8, TRUE);

-- Perfume Feminino Floral
INSERT INTO products (id, name, url_name, description, image, brand_id, category_id, published)
VALUES ('018e15ba-ff46-7023-549b-bffb6d3519e4', 'Perfume Feminino Floral', 'perfume-feminino-floral', 'Perfume feminino com aroma floral suave e notas delicadas.', 'perfume_feminino.jpg', 5, 9, TRUE);
