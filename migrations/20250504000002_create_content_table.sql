-- Create content table
CREATE TABLE IF NOT EXISTS content (
    id UUID PRIMARY KEY,
    content VARCHAR(2000) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    sender_id VARCHAR(255) NOT NULL,
    image_url VARCHAR(255),
    community_id VARCHAR(255) NOT NULL
);