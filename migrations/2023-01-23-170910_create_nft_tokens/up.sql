CREATE TABLE evm_nft_tokens (
  address TEXT NOT NULL,
  chain TEXT NOT NULL,
  token_type TEXT NOT NULL,
  name TEXT,
  symbol TEXT,
  contract_uri TEXT,
  PRIMARY KEY (address, chain)
);

CREATE INDEX IF NOT EXISTS evm_nft_tokens_by_address
ON evm_nft_tokens (address);

CREATE INDEX IF NOT EXISTS evm_nft_tokens_by_chain
ON evm_nft_tokens (chain);