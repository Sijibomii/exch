-- Your SQL goes here

CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";
CREATE TABLE wallet
(
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    balance BIGINT NOT NULL,
    user_id uuid NOT NULL,
    last_activity_time TIMESTAMPTZ
);

CREATE TABLE client 
(
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    next_id BIGINT NOT NULL,
    next_ticker_id BIGINT NOT NULL
);

CREATE TABLE token_ownership
(
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    wallet_id uuid NOT NULL,
    token_id uuid NOT NULL,
    balance BIGINT NOT NULL
);
 