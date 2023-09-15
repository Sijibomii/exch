-- Your SQL goes here
CREATE EXTENSION
IF NOT EXISTS "uuid-ossp";
CREATE TABLE tokens
(
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    ticker VARCHAR UNIQUE NOT NULL,
    is_trading BOOLEAN NOT NULL DEFAULT false,
    supply BIGINT,
    user_id uuid NOT NULL
);
