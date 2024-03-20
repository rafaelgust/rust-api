-- Your SQL goes here

CREATE TABLE feedbacks (
    id SERIAL PRIMARY KEY,
    date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    published BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_feedbacks_product_user ON feedbacks (product_id, user_id);

CREATE TABLE feedback_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE
);

CREATE TABLE grades (
    id SERIAL PRIMARY KEY,
    feedback_id INTEGER NOT NULL REFERENCES feedbacks(id) ON DELETE CASCADE,
    type_id INTEGER NOT NULL REFERENCES feedback_types(id),
    value INTEGER NOT NULL CHECK (value >= 0 AND value <= 10),
    FOREIGN KEY (feedback_id) REFERENCES feedbacks(id) ON DELETE CASCADE
);

INSERT INTO feedback_types (name) VALUES 
    ('Tonalidade'),
    ('Cheiro'),
    ('Qualidade'),
    ('Praticidade');