-- Add up migration script here

DROP TABLE IF EXISTS transactions;
CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    "description" VARCHAR(255) NOT NULL,
    merchant_name VARCHAR(255) NOT NULL,
    "value" REAL NOT NULL,
    currency VARCHAR(10) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);