-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    xid VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) NOT NULL,
    profile_image_url VARCHAR(255),
    wallet_address VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true
);

-- Create communities table
CREATE TABLE IF NOT EXISTS communities (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    creator_id UUID NOT NULL,
    creator_xid VARCHAR(255) NOT NULL,
    last_message_time TIMESTAMPTZ,
    contract_address VARCHAR(255),
    bounty_amount DECIMAL(20, 8) NOT NULL DEFAULT 0,
    time_limit INTEGER,
    base_fee_percentage REAL,
    wallet_address VARCHAR(255),
    image_url VARCHAR(255),
    FOREIGN KEY (creator_id) REFERENCES users(id),
    FOREIGN KEY (creator_xid) REFERENCES users(xid)
);

-- Create content table
CREATE TABLE IF NOT EXISTS content (
    id UUID PRIMARY KEY,
    content VARCHAR(2000) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    sender_id UUID NOT NULL,
    sender_xid VARCHAR(255) NOT NULL,
    image_url VARCHAR(255),
    community_id UUID NOT NULL,
    wallet_address VARCHAR(255) NOT NULL,
    FOREIGN KEY (sender_id) REFERENCES users(id),
    FOREIGN KEY (sender_xid) REFERENCES users(xid),
    FOREIGN KEY (community_id) REFERENCES communities(id)
);

-- Create depositor table
CREATE TABLE IF NOT EXISTS depositor (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    user_xid VARCHAR(255) NOT NULL,
    community_id UUID NOT NULL,
    amount DECIMAL(20, 8) NOT NULL,
    wallet_address VARCHAR(255),
    deposited_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (user_xid) REFERENCES users(xid),
    FOREIGN KEY (community_id) REFERENCES communities(id)
);