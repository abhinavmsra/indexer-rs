CREATE UNIQUE INDEX
  evm_logs_unique_on_transaction_hash_log_index
ON evm_logs (
  transaction_hash,
  log_index
);
