CREATE TABLE IF NOT EXISTS evm_chains
(
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    last_synced_block_number BIGINT NULL,
    block_time INTEGER NOT NULL,

    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_evm_chains_updated_at
BEFORE UPDATE ON evm_chains
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
