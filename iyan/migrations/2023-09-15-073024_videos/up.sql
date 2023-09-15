-- Your SQL goes here
CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";
CREATE TABLE users
(
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL,
    salt VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verification_token uuid,
    verification_token_expires_at TIMESTAMPTZ,
    reset_token uuid,
    reset_token_expires_at TIMESTAMPTZ,
    last_login_time TIMESTAMPTZ,
    last_login_ip VARCHAR,
    trading_client_id BIGINT,
    last_order_id BIGINT,
    last_seq_num BIGINT
);
