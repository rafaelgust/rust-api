-- Your SQL goes here

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    text VARCHAR(256) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    product_id UUID NOT NULL,
    user_id UUID NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    published BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_comments_product_user ON comments (product_id, user_id);
