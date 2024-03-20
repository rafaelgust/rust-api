-- Your SQL goes here

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(16) NOT NULL UNIQUE
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(32) NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL,
    email VARCHAR(64) NOT NULL UNIQUE,
    role_id INTEGER NOT NULL REFERENCES roles(id),
    published BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO roles (name) VALUES 
    ('Admin'),
    ('Manager'),
    ('Moderator'),
    ('User');