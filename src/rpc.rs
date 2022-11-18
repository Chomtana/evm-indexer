use anyhow::Result;
use web3::{
    futures::StreamExt,
    transports::{Batch, Http, WebSocket},
    types::{BlockId, H256, U64},
    Error, Web3,
};

use crate::db::IndexerDB;

pub struct IndexerRPC {
    pub batch: Web3<Batch<Http>>,
    pub wss: Web3<WebSocket>,
}

impl IndexerRPC {
    pub async fn new(rpc_ws_url: &str, rpc_http_url: &str) -> Result<Self> {
        log::info!("==> IndexerRPC: Initializing IndexerRPC");

        let http = Http::new(rpc_http_url)?;
        let ws = WebSocket::new(rpc_ws_url).await?;

        Ok(IndexerRPC {
            wss: Web3::new(ws),
            batch: Web3::new(web3::transports::Batch::new(http)),
        })
    }

    pub async fn last_block(&self) -> Result<i64> {
        let last_block = self
            .wss
            .eth()
            .block_number()
            .await
            .expect("Unable to fetch last block")
            .as_u64();

        Ok(last_block as i64)
    }

    pub async fn fetch_block_batch(
        &self,
        range: &[i64],
    ) -> Result<Vec<Result<serde_json::Value, Error>>> {
        for block_height in range.iter() {
            let block_number = U64::from_str_radix(&block_height.to_string(), 10)
                .expect("Unable to parse block number");

            let block_id = <BlockId as From<U64>>::from(block_number);

            self.batch.eth().block_with_txs(block_id);
        }

        let result = self.batch.transport().submit_batch().await;

        match result {
            Ok(result) => Ok(result),
            Err(_) => Ok(Vec::new()),
        }
    }

    pub async fn subscribe_heads(&self, db: &IndexerDB) {
        let mut sub = self
            .wss
            .eth_subscribe()
            .subscribe_new_heads()
            .await
            .unwrap();

        log::info!(
            "==> IndexerRPC: Initializing new heads listener with id {:?}",
            sub.id()
        );

        loop {
            let new_block = sub.next().await;

            match new_block {
                Some(block_header) => match block_header {
                    Ok(block_header) => {
                        log::info!(
                            "==> IndexerRPC: New block header with height {:?}",
                            block_header.number.unwrap()
                        );

                        let block_id = <BlockId as From<H256>>::from(block_header.hash.unwrap());

                        let block = self
                            .wss
                            .eth()
                            .block_with_txs(block_id)
                            .await
                            .unwrap()
                            .expect("Unable to deserialize block response");

                        db.store_block(block).await;
                    }
                    Err(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }
        }
    }
}
