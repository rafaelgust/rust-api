-- Your SQL goes here

CREATE TABLE categories (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(32) NOT NULL,
    url_name VARCHAR(256) NOT NULL UNIQUE,
    description VARCHAR(256) NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);