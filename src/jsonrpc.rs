use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind, RpcRequestValidationErrorKind};
use near_jsonrpc_primitives::message::{self, from_slice, Message};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockId, BlockReference, MaybeBlockId, ShardId};
use near_primitives::views;

use super::NearClient;

#[derive(Debug, Serialize)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

pub enum ExperimentalJsonRpcMethod {
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

#[cfg(feature = "sandbox")]
pub enum SandboxJsonRpcMethod {
    PatchState(near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateRequest),
}

#[cfg(feature = "adversarial")]
pub enum AdversarialJsonRpcMethod {
    SetWeight(u64),
    DisableHeaderSync,
    DisableDoomslug,
    ProduceBlocks { num_blocks: u64, only_valid: bool },
    SwitchToHeight(u64),
    GetSavedBlocks,
    CheckStore,
}

pub enum JsonRpcMethod {
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
    Experimental(ExperimentalJsonRpcMethod),
    #[cfg(feature = "sandbox")]
    Sandbox(SandboxJsonRpcMethod),
    #[cfg(feature = "adversarial")]
    Adversarial(AdversarialJsonRpcMethod),
}

#[cfg(feature = "adversarial")]
use AdversarialJsonRpcMethod::*;
use ExperimentalJsonRpcMethod::*;
use JsonRpcMethod::*;
#[cfg(feature = "sandbox")]
use SandboxJsonRpcMethod::*;

#[derive(Debug)]
pub enum JsonRpcTransportSendError {
    PayloadSerializeError(serde_json::Error),
    PayloadSendError(reqwest::Error),
}

#[derive(Debug)]
pub enum JsonRpcTransportHandlerResponseError {
    ResultParseError(serde_json::Error),
    ErrorMessageParseError(serde_json::Error),
}

#[derive(Debug)]
pub enum JsonRpcTransportRecvError {
    UnexpectedServerResponse(Message),
    // error occurred while retrieving payload
    PayloadRecvError(reqwest::Error),
    // invalid message from server
    PayloadParseError(message::Broken),
    // error while parsing response from method call
    ResponseParseError(JsonRpcTransportHandlerResponseError),
}

#[derive(Debug)]
pub enum RpcTransportError {
    SendError(JsonRpcTransportSendError),
    RecvError(JsonRpcTransportRecvError),
}

#[derive(Debug)]
pub enum JsonRpcServerError<E> {
    RequestValidationError(RpcRequestValidationErrorKind),
    HandlerError(E),
    InternalError(serde_json::Value),
}

#[derive(Debug)]
pub enum JsonRpcError<E> {
    TransportError(RpcTransportError),
    ServerError(JsonRpcServerError<E>),
}

type MethodExecutionError = RpcError;

pub type JsonRpcMethodCallResult<T> = Result<T, JsonRpcError<MethodExecutionError>>;

impl JsonRpcMethod {
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
            #[cfg(feature = "sandbox")]
            Sandbox(method) => match method {
                PatchState(request) => ("sandbox_patch_state", json!(request)),
            },
            #[cfg(feature = "adversarial")]
            Adversarial(method) => match method {
                SetWeight(height) => ("adv_set_weight", json!(height)),
                DisableHeaderSync => ("adv_disable_header_sync", json!(null)),
                DisableDoomslug => ("adv_disable_doomslug", json!(null)),
                ProduceBlocks {
                    num_blocks,
                    only_valid,
                } => ("adv_produce_blocks", json!([num_blocks, only_valid])),
                SwitchToHeight(height) => ("adv_switch_to_height", json!([height])),
                GetSavedBlocks => ("adv_get_saved_blocks", json!(null)),
                CheckStore => ("adv_check_store", json!(null)),
            },
        }
    }

    pub async fn call_on<T: DeserializeOwned>(
        &self,
        rpc_client: &NearJsonRpcClient,
    ) -> JsonRpcMethodCallResult<T> {
        let (method_name, params) = self.method_and_params();
        let request_payload = Message::request(method_name.to_string(), Some(params));
        let request_payload = serde_json::to_vec(&request_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSerializeError(err),
            ))
        })?;
        let near_client = &rpc_client.near_client;
        let request = near_client
            .client
            .post(&near_client.server_addr)
            .header("Content-Type", "application/json")
            .body(request_payload);
        let response = request.send().await.map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSendError(err),
            ))
        })?;
        let response_payload = response.bytes().await.map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::RecvError(
                JsonRpcTransportRecvError::PayloadRecvError(err),
            ))
        })?;
        let response_message = from_slice(&response_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::RecvError(
                JsonRpcTransportRecvError::PayloadParseError(err),
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
                        JsonRpcError::ServerError(JsonRpcServerError::HandlerError(
                            serde_json::from_value(handler_error).map_err(|err| {
                                JsonRpcError::TransportError(RpcTransportError::RecvError(
                                    JsonRpcTransportRecvError::ResponseParseError(
                                        JsonRpcTransportHandlerResponseError::ErrorMessageParseError(
                                            err,
                                        ),
                                    ),
                                ))
                            })?,
                        ))
                    }
                    RpcErrorKind::RequestValidationError(err) => {
                        JsonRpcError::ServerError(JsonRpcServerError::RequestValidationError(err))
                    }
                    RpcErrorKind::InternalError(err) => {
                        JsonRpcError::ServerError(JsonRpcServerError::InternalError(err))
                    }
                };
                Err(err)
            })?;
            return serde_json::from_value(response_result).map_err(|err| {
                JsonRpcError::TransportError(RpcTransportError::RecvError(
                    JsonRpcTransportRecvError::ResponseParseError(
                        JsonRpcTransportHandlerResponseError::ResultParseError(err),
                    ),
                ))
            });
        }
        Err(JsonRpcError::TransportError(RpcTransportError::RecvError(
            JsonRpcTransportRecvError::UnexpectedServerResponse(response_message),
        )))
    }
}

pub struct NearJsonRpcClient {
    pub(crate) near_client: NearClient,
}

impl NearJsonRpcClient {
    pub async fn broadcast_tx_async(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<String> {
        BroadcastTxAsync { tx }.call_on(self).await
    }

    pub async fn broadcast_tx_commit(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<views::FinalExecutionOutcomeView> {
        BroadcastTxCommit { tx }.call_on(self).await
    }

    pub async fn status(&self) -> JsonRpcMethodCallResult<views::StatusResponse> {
        Status.call_on(self).await
    }

    pub async fn health(&self) -> JsonRpcMethodCallResult<()> {
        Health.call_on(self).await
    }

    pub async fn tx(
        &self,
        hash: CryptoHash,
        id: AccountId,
    ) -> JsonRpcMethodCallResult<views::FinalExecutionOutcomeView> {
        Tx { hash, id }.call_on(self).await
    }

    pub async fn chunk(&self, id: ChunkId) -> JsonRpcMethodCallResult<views::ChunkView> {
        Chunk { id }.call_on(self).await
    }

    pub async fn validators(
        &self,
        block_id: MaybeBlockId,
    ) -> JsonRpcMethodCallResult<views::EpochValidatorInfo> {
        Validators { block_id }.call_on(self).await
    }

    pub async fn gas_price(
        &self,
        block_id: MaybeBlockId,
    ) -> JsonRpcMethodCallResult<views::GasPriceView> {
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

    pub async fn block(
        &self,
        request: BlockReference,
    ) -> JsonRpcMethodCallResult<views::BlockView> {
        Block(request).call_on(self).await
    }

    pub async fn light_client_proof(
        &self,
        request: near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofRequest,
    ) -> JsonRpcMethodCallResult<
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
    ) -> JsonRpcMethodCallResult<near_client_primitives::types::NetworkInfoResponse> {
        NetworkInfo.call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_check_tx(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<serde_json::Value> {
        Experimental(CheckTx { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_genesis_config(&self) -> JsonRpcMethodCallResult<serde_json::Value> {
        Experimental(GenesisConfig).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_broadcast_tx_sync(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<serde_json::Value> {
        Experimental(BroadcastTxSync { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_tx_status(
        &self,
        tx: String,
    ) -> JsonRpcMethodCallResult<serde_json::Value> {
        Experimental(TxStatus { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockRequest,
    ) -> JsonRpcMethodCallResult<near_jsonrpc_primitives::types::changes::RpcStateChangesResponse>
    {
        Experimental(Changes(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes_in_block(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockResponse,
    > {
        Experimental(ChangesInBlock(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_validators_ordered(
        &self,
        request: near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest,
    ) -> JsonRpcMethodCallResult<Vec<views::validator_stake_view::ValidatorStakeView>> {
        Experimental(ValidatorsOrdered(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_receipt(
        &self,
        request: near_jsonrpc_primitives::types::receipts::RpcReceiptRequest,
    ) -> JsonRpcMethodCallResult<near_jsonrpc_primitives::types::receipts::RpcReceiptResponse> {
        Experimental(Receipt(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_protocol_config(
        &self,
        request: near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest,
    ) -> JsonRpcMethodCallResult<near_jsonrpc_primitives::types::config::RpcProtocolConfigResponse>
    {
        Experimental(ProtocolConfig(request)).call_on(self).await
    }

    #[cfg(feature = "sandbox")]
    pub async fn sandbox_patch_state(
        &self,
        request: near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateResponse,
    > {
        Sandbox(PatchState(request)).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_set_weight(&self, height: u64) -> JsonRpcMethodCallResult<serde_json::Value> {
        Adversarial(SetWeight(height)).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_disable_header_sync(&self) -> JsonRpcMethodCallResult<serde_json::Value> {
        Adversarial(DisableHeaderSync).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_disable_doomslug(&self) -> JsonRpcMethodCallResult<serde_json::Value> {
        Adversarial(DisableDoomslug).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_produce_blocks(
        &self,
        num_blocks: u64,
        only_valid: bool,
    ) -> JsonRpcMethodCallResult<serde_json::Value> {
        Adversarial(ProduceBlocks {
            num_blocks,
            only_valid,
        })
        .call_on(self)
        .await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_switch_to_height(
        &self,
        height: u64,
    ) -> JsonRpcMethodCallResult<serde_json::Value> {
        Adversarial(SwitchToHeight(height)).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_get_saved_blocks(&self) -> JsonRpcMethodCallResult<serde_json::Value> {
        Adversarial(GetSavedBlocks).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_check_store(&self) -> JsonRpcMethodCallResult<serde_json::Value> {
        Adversarial(CheckStore).call_on(self).await
    }
}