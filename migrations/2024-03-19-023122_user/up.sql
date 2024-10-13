CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(16) NOT NULL UNIQUE
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
    username VARCHAR(32) NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL,
    email VARCHAR(64) NOT NULL UNIQUE,
    role_id INTEGER NOT NULL REFERENCES roles(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    published BOOLEAN NOT NULL DEFAULT FALSE
);

-- Inserindo valores na tabela roles
INSERT INTO roles (name) VALUES 
    ('Admin'),
    ('Manager'),
    ('Moderator'),
    ('User');

-- Inserindo valores na tabela users
INSERT INTO users (id, first_name, last_name, username, password, email, role_id, published)
VALUES ('018e1124-9ed7-73d5-a268-385297389888', 'Admin', 'User', 'admin_user', '$2b$12$ZdRih81d4Q67xQi3HO21Ie0rYft/o60SeQEnLSu0aFzNABRbou4L2', 'admin@example.com', 1, TRUE);

INSERT INTO users (id, first_name, last_name, username, password, email, role_id, published)
VALUES ('018e1116-df65-7380-aae2-2ac903fe61a4', 'Manager', 'User', 'manager_user', '$2b$12$ZdRih81d4Q67xQi3HO21Ie0rYft/o60SeQEnLSu0aFzNABRbou4L2', 'manager@example.com', 2, TRUE);

INSERT INTO users (id, first_name, last_name, username, password, email, role_id, published)
VALUES ('018e1116-df66-7232-a114-50bc2b13daf0', 'Moderator', 'User', 'moderator_user', '$2b$12$ZdRih81d4Q67xQi3HO21Ie0rYft/o60SeQEnLSu0aFzNABRbou4L2', 'moderator@example.com', 3, TRUE);

INSERT INTO users (id, first_name, last_name, username, password, email, role_id, published)
VALUES ('018e1116-df66-7237-a115-51bc2b13def0', 'Regular', 'User', 'regular_user', '$2b$12$ZdRih81d4Q67xQi3HO21Ie0rYft/o60SeQEnLSu0aFzNABRbou4L2', 'user@example.com', 4, TRUE);