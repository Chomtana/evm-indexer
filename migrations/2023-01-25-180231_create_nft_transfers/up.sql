CREATE TABLE evm_nft_transfers (
  hash TEXT NOT NULL, 
  log_index BIGINT NOT NULL,
  transfer_index BIGINT NOT NULL,
  event_type TEXT NOT NULL,
  token TEXT NOT NULL,
  from_address TEXT NOT NULL,
  to_address TEXT NOT NULL,
  token_id TEXT NOT NULL,
  value TEXT NOT NULL,
  nft_tokens_parsed BOOL,
  CONSTRAINT nft_transfers_table_pk PRIMARY KEY (hash, log_index, transfer_index)
);

CREATE INDEX IF NOT EXISTS evm_nft_transfers_by_token
ON evm_nft_transfers (token, token_id);

CREATE INDEX IF NOT EXISTS evm_nft_transfers_by_hash 
ON evm_nft_transfers (hash);

CREATE INDEX IF NOT EXISTS evm_nft_transfers_by_sender
ON evm_nft_transfers (from_address);

CREATE INDEX IF NOT EXISTS evm_nft_transfers_by_receiver
ON evm_nft_transfers (to_address);
