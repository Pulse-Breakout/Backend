-- Create depositor table
CREATE TABLE IF NOT EXISTS depositor (
    id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    community_id VARCHAR(255) NOT NULL,
    amount DECIMAL(20, 8) NOT NULL,
    wallet_address VARCHAR(255),
    deposited_at TIMESTAMPTZ NOT NULL
);