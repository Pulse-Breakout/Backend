-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    xid VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL,
    profile_image_url VARCHAR(255),
    wallet_address VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);