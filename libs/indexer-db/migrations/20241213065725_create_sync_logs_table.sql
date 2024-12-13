CREATE TABLE IF NOT EXISTS evm_sync_logs
(
    address BYTEA PRIMARY KEY,
    last_synced_block_number BIGINT NOT NULL DEFAULT 0,

    chain_id BIGINT NOT NULL REFERENCES evm_chains(id),

    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_evm_sync_logs_updated_at
BEFORE UPDATE ON evm_sync_logs
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
