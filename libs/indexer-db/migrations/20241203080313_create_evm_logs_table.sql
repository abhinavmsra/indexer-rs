CREATE UNLOGGED TABLE IF NOT EXISTS evm_logs
(
    id SERIAL PRIMARY KEY,
    block_number NUMERIC NOT NULL,
    block_hash BYTEA NOT NULL,
    block_timestamp BIGINT NOT NULL,
    address BYTEA NOT NULL,
    transaction_hash BYTEA NOT NULL,
    transaction_index BIGINT NOT NULL,
    log_index BIGINT NOT NULL,
    removed BOOL DEFAULT FALSE,
    data BYTEA,
    event_signature BYTEA,
    topics BYTEA[],

    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);
