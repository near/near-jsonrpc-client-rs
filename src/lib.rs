#![deprecated(note = "This crate is unstable and hence, unfit for use.")]
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

use near_jsonrpc_primitives::errors::RpcError;
use near_jsonrpc_primitives::message::{from_slice, Message};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockId, BlockReference, MaybeBlockId, ShardId};
use near_primitives::views;

#[derive(Debug, Serialize)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

pub enum ExperimentalRpcMethod {
    CheckTx {
        tx: views::SignedTransactionView,
    },
    GenesisConfig,
    BroadcastTxSync {
        tx: views::SignedTransactionView,
    },
    TxStatus {
        tx: String,
    },
    Changes {
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesRequest,
    },
    ValidatorsOrdered {
        request: near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest,
    },
    Receipt {
        request: near_jsonrpc_primitives::types::receipts::RpcReceiptRequest,
    },
    ProtocolConfig {
        request: near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest,
    },
}

pub enum RpcMethod {
    BroadcastTxAsync {
        tx: views::SignedTransactionView,
    },
    BroadcastTxCommit {
        tx: views::SignedTransactionView,
    },
    Status,
    Health,
    Tx {
        hash: CryptoHash,
        id: AccountId,
    },
    Chunk {
        id: ChunkId,
    },
    Validators {
        block_id: MaybeBlockId,
    },
    GasPrice {
        block_id: MaybeBlockId,
    },
    Query {
        request: near_jsonrpc_primitives::types::query::RpcQueryRequest,
    },
    Block {
        request: BlockReference,
    },
    Experimental(ExperimentalRpcMethod),
}

impl RpcMethod {
    fn method_and_params(&self) -> (&str, serde_json::Value) {
        match self {
            Self::BroadcastTxAsync { tx } => ("broadcast_tx_async", json!([tx])),
            Self::BroadcastTxCommit { tx } => ("broadcast_tx_commit", json!([tx])),
            Self::Status => ("status", json!([])),
            Self::Health => ("health", json!([])),
            Self::Tx { hash, id } => ("tx", json!([hash, id])),
            Self::Chunk { id } => ("chunk", json!([id])),
            Self::Validators { block_id } => ("validators", json!([block_id])),
            Self::GasPrice { block_id } => ("gas_price", json!([block_id])),
            Self::Query { request } => ("query", json!(request)),
            Self::Block { request } => ("block", json!(request)),
            Self::Experimental(ExperimentalRpcMethod::CheckTx { tx }) => {
                ("EXPERIMENTAL_check_tx", json!([tx]))
            }
            Self::Experimental(ExperimentalRpcMethod::GenesisConfig) => {
                ("EXPERIMENTAL_genesis_config", json!([]))
            }
            Self::Experimental(ExperimentalRpcMethod::BroadcastTxSync { tx }) => {
                ("EXPERIMENTAL_broadcast_tx_sync", json!([tx]))
            }
            Self::Experimental(ExperimentalRpcMethod::TxStatus { tx }) => {
                ("EXPERIMENTAL_tx_status", json!([tx]))
            }
            Self::Experimental(ExperimentalRpcMethod::Changes { request }) => {
                ("EXPERIMENTAL_changes", json!(request))
            }
            Self::Experimental(ExperimentalRpcMethod::ValidatorsOrdered { request }) => {
                ("EXPERIMENTAL_validators_ordered", json!(request))
            }
            Self::Experimental(ExperimentalRpcMethod::Receipt { request }) => {
                ("EXPERIMENTAL_receipt", json!(request))
            }
            Self::Experimental(ExperimentalRpcMethod::ProtocolConfig { request }) => {
                ("EXPERIMENTAL_protocol_config", json!(request))
            }
        }
    }

    pub async fn call_on<T: DeserializeOwned>(
        &self,
        rpc_client: &JsonRpcClient,
    ) -> Result<T, RpcError> {
        let (method_name, params) = self.method_and_params();
        let request_payload = Message::request(method_name.to_string(), Some(params));
        let request = rpc_client
            .client
            .post(&rpc_client.server_addr)
            .header("Content-Type", "application/json")
            .json(&request_payload);
        let response = request
            .send()
            .await
            .map_err(|err| RpcError::new_internal_error(None, format!("{:?}", err)))?;
        let response_payload = response.bytes().await.map_err(|err| {
            RpcError::parse_error(format!("Failed to retrieve response payload: {:?}", err))
        })?;
        if let Message::Response(response) = from_slice(&response_payload).map_err(|err| {
            RpcError::parse_error(format!("Failed parsing response payload: {:?}", err))
        })? {
            return serde_json::from_value(response.result?)
                .map_err(|err| RpcError::parse_error(format!("Failed to parse: {:?}", err)));
        }
        Err(RpcError::parse_error(format!(
            "Failed to parse JSON RPC response"
        )))
    }
}

#[derive(Debug, Clone)]
pub struct JsonRpcClient {
    server_addr: String,
    client: Client,
}

impl JsonRpcClient {
    pub fn new(server_addr: &str, client: &Client) -> Self {
        Self {
            server_addr: server_addr.to_string(),
            client: client.clone(),
        }
    }

    pub fn new_client(server_addr: &str) -> Self {
        Self {
            server_addr: server_addr.to_string(),
            client: Client::new(),
        }
    }

    pub async fn broadcast_tx_async(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<String, RpcError> {
        RpcMethod::BroadcastTxAsync { tx }.call_on(self).await
    }

    pub async fn broadcast_tx_commit(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<views::FinalExecutionOutcomeView, RpcError> {
        RpcMethod::BroadcastTxCommit { tx }.call_on(self).await
    }

    pub async fn status(&self) -> Result<views::StatusResponse, RpcError> {
        RpcMethod::Status.call_on(self).await
    }

    pub async fn health(&self) -> Result<(), RpcError> {
        RpcMethod::Health.call_on(self).await
    }

    pub async fn tx(
        &self,
        hash: CryptoHash,
        id: AccountId,
    ) -> Result<views::FinalExecutionOutcomeView, RpcError> {
        RpcMethod::Tx { hash, id }.call_on(self).await
    }

    pub async fn chunk(&self, id: ChunkId) -> Result<views::ChunkView, RpcError> {
        RpcMethod::Chunk { id }.call_on(self).await
    }

    pub async fn validators(
        &self,
        block_id: MaybeBlockId,
    ) -> Result<views::EpochValidatorInfo, RpcError> {
        RpcMethod::Validators { block_id }.call_on(self).await
    }

    pub async fn gas_price(&self, block_id: MaybeBlockId) -> Result<views::GasPriceView, RpcError> {
        RpcMethod::GasPrice { block_id }.call_on(self).await
    }

    pub async fn query(
        &self,
        request: near_jsonrpc_primitives::types::query::RpcQueryRequest,
    ) -> Result<near_jsonrpc_primitives::types::query::RpcQueryResponse, RpcError> {
        RpcMethod::Query { request }.call_on(self).await
    }

    pub async fn block(&self, request: BlockReference) -> Result<views::BlockView, RpcError> {
        RpcMethod::Block { request }.call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_check_tx(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<serde_json::Value, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::CheckTx { tx })
            .call_on(self)
            .await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_genesis_config(&self) -> Result<serde_json::Value, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::GenesisConfig)
            .call_on(self)
            .await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_broadcast_tx_sync(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<serde_json::Value, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::BroadcastTxSync { tx })
            .call_on(self)
            .await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_tx_status(&self, tx: String) -> Result<serde_json::Value, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::TxStatus { tx })
            .call_on(self)
            .await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesRequest,
    ) -> Result<near_jsonrpc_primitives::types::changes::RpcStateChangesResponse, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::Changes { request })
            .call_on(self)
            .await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_validators_ordered(
        &self,
        request: near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest,
    ) -> Result<Vec<views::validator_stake_view::ValidatorStakeView>, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::ValidatorsOrdered { request })
            .call_on(self)
            .await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_receipt(
        &self,
        request: near_jsonrpc_primitives::types::receipts::RpcReceiptRequest,
    ) -> Result<near_jsonrpc_primitives::types::receipts::RpcReceiptResponse, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::Receipt { request })
            .call_on(self)
            .await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_protocol_config(
        &self,
        request: near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest,
    ) -> Result<near_jsonrpc_primitives::types::config::RpcProtocolConfigResponse, RpcError> {
        RpcMethod::Experimental(ExperimentalRpcMethod::ProtocolConfig { request })
            .call_on(self)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{JsonRpcClient, RpcMethod};

    #[tokio::test]
    async fn it_works() {
        let rpc_client = JsonRpcClient::new_client("http://localhost:3030");
        let status1 = rpc_client.status().await;
        let status2 = RpcMethod::Status
            .call_on::<near_primitives::views::StatusResponse>(&rpc_client)
            .await;

        println!("{:?}", status1);
        println!("{:?}", status2);
    }
}
