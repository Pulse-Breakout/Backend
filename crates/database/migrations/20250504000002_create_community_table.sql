-- Create communities table
CREATE TABLE IF NOT EXISTS communities (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    creator_id VARCHAR(255) NOT NULL,
    last_message_time TIMESTAMPTZ,
    contract_address VARCHAR(255),
    bounty_amount DECIMAL NOT NULL DEFAULT 0,
    time_limit INTEGER,
    base_fee_percentage REAL,
    wallet_address VARCHAR(255),
    image_url VARCHAR(255)
);