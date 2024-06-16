-- Your SQL goes here

CREATE TABLE brands (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(128) NOT NULL,
    url_name VARCHAR(256) NOT NULL UNIQUE,
    description VARCHAR(512) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    published BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO brands (name, url_name, description, published)
VALUES ('Apple', 'apple', 'Leading technology company known for iPhone, iPad, and Mac.', TRUE);

INSERT INTO brands (name, url_name, description, published)
VALUES ('Samsung', 'samsung', 'Global electronics company known for smartphones and home appliances.', TRUE);

INSERT INTO brands (name, url_name, description, published)
VALUES ('Sony', 'sony', 'Japanese multinational conglomerate known for electronics, gaming, and entertainment.', TRUE);

INSERT INTO brands (name, url_name, description, published)
VALUES ('Dell', 'dell', 'American technology company that develops, sells, and supports computers and related products.', TRUE);

INSERT INTO brands (name, url_name, description, published)
VALUES ('Nike', 'nike', 'Global brand specializing in athletic footwear, apparel, and equipment.', TRUE);
