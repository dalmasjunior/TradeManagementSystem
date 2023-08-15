-- Your SQL goes here
CREATE TABLE IF NOT EXISTS wallet (
    id CHARACTER(36) PRIMARY KEY NOT NULL,
    hash VARCHAR(255) NOT NULL,
    balance REAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS users (
    id CHARACTER(36) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    wallet_id CHARACTER(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (wallet_id) REFERENCES wallet(id)
);

CREATE TABLE IF NOT EXISTS trades (
    id CHARACTER(36) PRIMARY KEY NOT NULL,
    user_id CHARACTER(36) NOT NULL,
    wallet_id CHARACTER(36) NOT NULL,
    amount REAL NOT NULL,
    chain VARCHAR(20) NOT NULL,
    trade_type VARCHAR(20) NOT NULL,
    asset VARCHAR(5) NOT NULL,
    before_price REAL NOT NULL,
    execution_price REAL NOT NULL,
    final_price REAL NOT NULL,
    traded_amount REAL NOT NULL,
    execution_fee REAL NOT NULL,
    transaction_fee REAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (wallet_id) REFERENCES wallet(id)
);

