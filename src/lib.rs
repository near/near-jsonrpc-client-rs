#![deprecated(note = "this crate is unstable and hence, unfit for use.")]
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind, RpcRequestValidationErrorKind};
use near_jsonrpc_primitives::message::{self, from_slice, Message};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockId, BlockReference, MaybeBlockId, ShardId};
use near_primitives::views;

#[derive(Debug, Serialize)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

#[derive(Debug)]
pub enum RpcTransportSendError {
    PayloadSerializeError(serde_json::Error),
    PayloadSendError(reqwest::Error),
}

#[derive(Debug)]
pub enum RpcTransportHandlerResponseError {
    ResultParseError(serde_json::Error),
    ErrorMessageParseError(serde_json::Error),
}

#[derive(Debug)]
pub enum RpcTransportRecvError {
    UnexpectedServerResponse(Message),
    // error occurred while retrieving payload
    PayloadRecvError(reqwest::Error),
    // invalid message from server
    PayloadParseError(message::Broken),
    // error while parsing response from method call
    ResponseParseError(RpcTransportHandlerResponseError),
}

#[derive(Debug)]
pub enum RpcTransportError {
    SendError(RpcTransportSendError),
    RecvError(RpcTransportRecvError),
}

#[derive(Debug)]
pub enum JsonRpcError<E> {
    TransportError(RpcTransportError),
    ServerError(RpcServerError<E>),
}

#[derive(Debug)]
pub enum RpcServerError<E> {
    RequestValidationError(RpcRequestValidationErrorKind),
    HandlerError(E),
    InternalError(serde_json::Value),
}

type MethodExecutionError = RpcError;

pub type RpcMethodCallResult<T> = Result<T, JsonRpcError<MethodExecutionError>>;

pub enum ExperimentalRpcMethod {
    CheckTx { tx: views::SignedTransactionView },
    GenesisConfig,
    BroadcastTxSync { tx: views::SignedTransactionView },
    TxStatus { tx: String },
    ChangesInBlock(near_jsonrpc_primitives::types::changes::RpcStateChangesRequest),
    Changes(near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockRequest),
    ValidatorsOrdered(near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest),
    Receipt(near_jsonrpc_primitives::types::receipts::RpcReceiptRequest),
    ProtocolConfig(near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest),
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
    Query(near_jsonrpc_primitives::types::query::RpcQueryRequest),
    Block(BlockReference),
    LightClientProof(
        near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofRequest,
    ),
    NextLightClientBlock(
        near_jsonrpc_primitives::types::light_client::RpcLightClientNextBlockRequest,
    ),
    NetworkInfo,
    Experimental(ExperimentalRpcMethod),
}

use ExperimentalRpcMethod::*;
use RpcMethod::*;

impl RpcMethod {
    fn method_and_params(&self) -> (&str, serde_json::Value) {
        match self {
            BroadcastTxAsync { tx } => ("broadcast_tx_async", json!([tx])),
            BroadcastTxCommit { tx } => ("broadcast_tx_commit", json!([tx])),
            Status => ("status", json!(null)),
            Health => ("health", json!(null)),
            Tx { hash, id } => ("tx", json!([hash, id])),
            Chunk { id } => ("chunk", json!([id])),
            Validators { block_id } => ("validators", json!([block_id])),
            GasPrice { block_id } => ("gas_price", json!([block_id])),
            Query(request) => ("query", json!(request)),
            Block(request) => ("block", json!(request)),
            LightClientProof(request) => ("light_client_proof", json!(request)),
            NextLightClientBlock(request) => ("next_light_client_block", json!(request)),
            NetworkInfo => ("network_info", json!(null)),
            Experimental(method) => match method {
                CheckTx { tx } => ("EXPERIMENTAL_check_tx", json!([tx])),
                GenesisConfig => ("EXPERIMENTAL_genesis_config", json!(null)),
                BroadcastTxSync { tx } => ("EXPERIMENTAL_broadcast_tx_sync", json!([tx])),
                TxStatus { tx } => ("EXPERIMENTAL_tx_status", json!([tx])),
                Changes(request) => ("EXPERIMENTAL_changes", json!(request)),
                ChangesInBlock(request) => ("EXPERIMENTAL_changes_in_block", json!(request)),
                ValidatorsOrdered(request) => ("EXPERIMENTAL_validators_ordered", json!(request)),
                Receipt(request) => ("EXPERIMENTAL_receipt", json!(request)),
                ProtocolConfig(request) => ("EXPERIMENTAL_protocol_config", json!(request)),
            },
        }
    }

    pub async fn call_on<T: DeserializeOwned>(
        &self,
        rpc_client: &JsonRpcClient,
    ) -> RpcMethodCallResult<T> {
        let (method_name, params) = self.method_and_params();
        let request_payload = Message::request(method_name.to_string(), Some(params));
        let request_payload = serde_json::to_vec(&request_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                RpcTransportSendError::PayloadSerializeError(err),
            ))
        })?;
        let request = rpc_client
            .client
            .post(&rpc_client.server_addr)
            .header("Content-Type", "application/json")
            .body(request_payload);
        let response = request.send().await.map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                RpcTransportSendError::PayloadSendError(err),
            ))
        })?;
        let response_payload = response.bytes().await.map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::RecvError(
                RpcTransportRecvError::PayloadRecvError(err),
            ))
        })?;
        let response_message = from_slice(&response_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::RecvError(
                RpcTransportRecvError::PayloadParseError(err),
            ))
        })?;
        if let Message::Response(response) = response_message {
            let response_result = response.result.or_else(|err| {
                let err = match if err.error_struct.is_some() {
                    err
                } else {
                    RpcError::new_internal_error(None, format!("<no data>"))
                }
                .error_struct
                .unwrap()
                {
                    RpcErrorKind::HandlerError(handler_error) => {
                        JsonRpcError::ServerError(RpcServerError::HandlerError(
                            serde_json::from_value(handler_error).map_err(|err| {
                                JsonRpcError::TransportError(RpcTransportError::RecvError(
                                    RpcTransportRecvError::ResponseParseError(
                                        RpcTransportHandlerResponseError::ErrorMessageParseError(
                                            err,
                                        ),
                                    ),
                                ))
                            })?,
                        ))
                    }
                    RpcErrorKind::RequestValidationError(err) => {
                        JsonRpcError::ServerError(RpcServerError::RequestValidationError(err))
                    }
                    RpcErrorKind::InternalError(err) => {
                        JsonRpcError::ServerError(RpcServerError::InternalError(err))
                    }
                };
                Err(err)
            })?;
            return serde_json::from_value(response_result).map_err(|err| {
                JsonRpcError::TransportError(RpcTransportError::RecvError(
                    RpcTransportRecvError::ResponseParseError(
                        RpcTransportHandlerResponseError::ResultParseError(err),
                    ),
                ))
            });
        }
        Err(JsonRpcError::TransportError(RpcTransportError::RecvError(
            RpcTransportRecvError::UnexpectedServerResponse(response_message),
        )))
    }
}

#[derive(Clone)]
pub struct JsonRpcClientBuilder {
    client: reqwest::Client,
}

impl JsonRpcClientBuilder {
    pub fn connect(&self, server_addr: &str) -> JsonRpcClient {
        JsonRpcClient {
            server_addr: server_addr.to_string(),
            client: self.client.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonRpcClient {
    server_addr: String,
    client: reqwest::Client,
}

impl JsonRpcClient {
    pub fn new_client() -> JsonRpcClientBuilder {
        JsonRpcClientBuilder {
            client: reqwest::Client::new(),
        }
    }

    pub async fn broadcast_tx_async(
        &self,
        tx: views::SignedTransactionView,
    ) -> RpcMethodCallResult<String> {
        BroadcastTxAsync { tx }.call_on(self).await
    }

    pub async fn broadcast_tx_commit(
        &self,
        tx: views::SignedTransactionView,
    ) -> RpcMethodCallResult<views::FinalExecutionOutcomeView> {
        BroadcastTxCommit { tx }.call_on(self).await
    }

    pub async fn status(&self) -> RpcMethodCallResult<views::StatusResponse> {
        Status.call_on(self).await
    }

    pub async fn health(&self) -> RpcMethodCallResult<()> {
        Health.call_on(self).await
    }

    pub async fn tx(
        &self,
        hash: CryptoHash,
        id: AccountId,
    ) -> RpcMethodCallResult<views::FinalExecutionOutcomeView> {
        Tx { hash, id }.call_on(self).await
    }

    pub async fn chunk(&self, id: ChunkId) -> RpcMethodCallResult<views::ChunkView> {
        Chunk { id }.call_on(self).await
    }

    pub async fn validators(
        &self,
        block_id: MaybeBlockId,
    ) -> RpcMethodCallResult<views::EpochValidatorInfo> {
        Validators { block_id }.call_on(self).await
    }

    pub async fn gas_price(
        &self,
        block_id: MaybeBlockId,
    ) -> RpcMethodCallResult<views::GasPriceView> {
        GasPrice { block_id }.call_on(self).await
    }

    pub async fn query(
        &self,
        request: near_jsonrpc_primitives::types::query::RpcQueryRequest,
    ) -> Result<
        near_jsonrpc_primitives::types::query::RpcQueryResponse,
        JsonRpcError<MethodExecutionError>,
    > {
        Query(request).call_on(self).await
    }

    pub async fn block(&self, request: BlockReference) -> RpcMethodCallResult<views::BlockView> {
        Block(request).call_on(self).await
    }

    pub async fn light_client_proof(
        &self,
        request: near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofRequest,
    ) -> RpcMethodCallResult<
        near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofResponse,
    > {
        LightClientProof(request).call_on(self).await
    }

    // todo: RpcLightClientNextBlockResponse doesn't impl Deserialize
    // pub async fn next_light_client_block(
    //     &self,
    //     request: near_jsonrpc_primitives::types::light_client::RpcLightClientNextBlockRequest,
    // ) -> RpcMethodCallResult<
    //     near_jsonrpc_primitives::types::light_client::RpcLightClientNextBlockResponse,
    // > {
    //     NextLightClientBlock(request).call_on(self).await
    // }

    pub async fn network_info(
        &self,
    ) -> RpcMethodCallResult<near_client_primitives::types::NetworkInfoResponse> {
        NetworkInfo.call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_check_tx(
        &self,
        tx: views::SignedTransactionView,
    ) -> RpcMethodCallResult<serde_json::Value> {
        Experimental(CheckTx { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_genesis_config(&self) -> RpcMethodCallResult<serde_json::Value> {
        Experimental(GenesisConfig).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_broadcast_tx_sync(
        &self,
        tx: views::SignedTransactionView,
    ) -> RpcMethodCallResult<serde_json::Value> {
        Experimental(BroadcastTxSync { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_tx_status(
        &self,
        tx: String,
    ) -> RpcMethodCallResult<serde_json::Value> {
        Experimental(TxStatus { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockRequest,
    ) -> RpcMethodCallResult<near_jsonrpc_primitives::types::changes::RpcStateChangesResponse> {
        Experimental(Changes(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes_in_block(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesRequest,
    ) -> RpcMethodCallResult<near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockResponse>
    {
        Experimental(ChangesInBlock(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_validators_ordered(
        &self,
        request: near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest,
    ) -> RpcMethodCallResult<Vec<views::validator_stake_view::ValidatorStakeView>> {
        Experimental(ValidatorsOrdered(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_receipt(
        &self,
        request: near_jsonrpc_primitives::types::receipts::RpcReceiptRequest,
    ) -> RpcMethodCallResult<near_jsonrpc_primitives::types::receipts::RpcReceiptResponse> {
        Experimental(Receipt(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_protocol_config(
        &self,
        request: near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest,
    ) -> RpcMethodCallResult<near_jsonrpc_primitives::types::config::RpcProtocolConfigResponse>
    {
        Experimental(ProtocolConfig(request)).call_on(self).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{JsonRpcClient, RpcMethod};

    #[tokio::test]
    async fn it_works() {
        let rpc_client = JsonRpcClient::new_client().connect("http://localhost:3030");
        let status1 = rpc_client.status().await;
        let status2 = RpcMethod::Status
            .call_on::<near_primitives::views::StatusResponse>(&rpc_client)
            .await;

        println!("{:?}", status1.unwrap());
        println!("{:?}", status2.unwrap());
    }
}
