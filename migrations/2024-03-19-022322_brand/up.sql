-- Your SQL goes here

CREATE TABLE brands (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(128) NOT NULL,
    url_name VARCHAR(256) NOT NULL UNIQUE,
    description VARCHAR(512) NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);
