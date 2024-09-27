-- Your SQL goes here
CREATE TABLE product_categories (
    product_id UUID NOT NULL,
    category_id INTEGER NOT NULL,
    PRIMARY KEY (product_id, category_id),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- Vestido de Festa Azul (associado às categorias 3 e 4, por exemplo)
INSERT INTO product_categories (product_id, category_id)
VALUES ('018e15ba-ff46-7023-545b-bffb6d3515e4', 3),
       ('018e15ba-ff46-7023-545b-bffb6d3515e4', 4);

-- Sapato Social Masculino (associado às categorias 2 e 6, por exemplo)
INSERT INTO product_categories (product_id, category_id)
VALUES ('018e15ba-ff46-7023-546b-bffb6d3516e4', 2),
       ('018e15ba-ff46-7023-546b-bffb6d3516e4', 6);

-- Bolsa Feminina de Couro (associado às categorias 5 e 7)
INSERT INTO product_categories (product_id, category_id)
VALUES ('018e15ba-ff46-7023-547b-bffb6d3517e4', 5),
       ('018e15ba-ff46-7023-547b-bffb6d3517e4', 7);

-- Perfume Masculino Intenso (associado às categorias 8 e 9)
INSERT INTO product_categories (product_id, category_id)
VALUES ('018e15ba-ff46-7023-548b-bffb6d3518e4', 8),
       ('018e15ba-ff46-7023-548b-bffb6d3518e4', 9);

-- Perfume Feminino Floral (associado às categorias 9 e 10)
INSERT INTO product_categories (product_id, category_id)
VALUES ('018e15ba-ff46-7023-549b-bffb6d3519e4', 9),
       ('018e15ba-ff46-7023-549b-bffb6d3519e4', 10);