CREATE UNLOGGED TABLE IF NOT EXISTS evm_logs
(
    id integer PRIMARY KEY,
    block_number numeric NOT NULL,
    transaction_hash bytea NOT NULL,
    address bytea NOT NULL,
    data jsonb
);
