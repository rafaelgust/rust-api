-- Your SQL goes here

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(16) NOT NULL UNIQUE
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(32) NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL,
    email VARCHAR(64) NOT NULL UNIQUE,
    role_id INTEGER NOT NULL REFERENCES roles(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    published BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO roles (name) VALUES 
    ('Admin'),
    ('Manager'),
    ('Moderator'),
    ('User');

INSERT INTO users (id, username, password, email, role_id, published)
VALUES ('018e1124-9ed7-73d5-a268-385297389888', 'admin_user', 'a7ec159e76d31f3d869712e677deb4b352e7cb6594838ba3cf4579f2a4490245', 'admin@example.com', 1, TRUE);

INSERT INTO users (id, username, password, email, role_id, published)
VALUES ('018e1116-df65-7380-aae2-2ac903fe61a4', 'manager_user', 'a7ec159e76d31f3d869712e677deb4b352e7cb6594838ba3cf4579f2a4490245', 'manager@example.com', 2, TRUE);

INSERT INTO users (id, username, password, email, role_id, published)
VALUES ('018e1116-df66-7232-a114-50bc2b13daf0', 'moderator_user', 'a7ec159e76d31f3d869712e677deb4b352e7cb6594838ba3cf4579f2a4490245', 'moderator@example.com', 3, TRUE);

INSERT INTO users (id, username, password, email, role_id, published)
VALUES ('018e1116-df66-7237-a115-51bc2b13def0', 'regular_user', 'a7ec159e76d31f3d869712e677deb4b352e7cb6594838ba3cf4579f2a4490245', 'user@example.com', 4, TRUE);