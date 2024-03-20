-- Your SQL goes here

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    text VARCHAR(256) NOT NULL,
    date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    product_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    published BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_comments_product_user ON comments (product_id, user_id);
