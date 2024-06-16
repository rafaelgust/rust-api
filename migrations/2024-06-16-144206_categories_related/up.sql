-- Your SQL goes here

CREATE TABLE categories_related (
    parent_id INTEGER NOT NULL,
    child_id INTEGER NOT NULL,
    PRIMARY KEY (parent_id, child_id),
    FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE CASCADE,
    FOREIGN KEY (child_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- Moda
INSERT INTO categories_related (parent_id, child_id) VALUES (1, 2); -- Moda -> Roupas Masculinas
INSERT INTO categories_related (parent_id, child_id) VALUES (1, 3); -- Moda -> Roupas Femininas
INSERT INTO categories_related (parent_id, child_id) VALUES (1, 4); -- Moda -> Acessórios
INSERT INTO categories_related (parent_id, child_id) VALUES (4, 5); -- Acessórios -> Bolsas
INSERT INTO categories_related (parent_id, child_id) VALUES (4, 6); -- Acessórios -> Jóias

-- Perfumes
INSERT INTO categories_related (parent_id, child_id) VALUES (7, 8); -- Perfumes -> Perfumes Masculinos
INSERT INTO categories_related (parent_id, child_id) VALUES (7, 9); -- Perfumes -> Perfumes Femininos
INSERT INTO categories_related (parent_id, child_id) VALUES (7, 10); -- Perfumes -> Perfumes Unissex
