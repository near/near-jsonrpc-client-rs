//! RPC API Client for the NEAR Protocol

use std::fmt;

use thiserror::Error;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind, RpcRequestValidationErrorKind};
use near_jsonrpc_primitives::message::{self, from_slice, Message};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockId, BlockReference, MaybeBlockId, ShardId};
use near_primitives::views;

use super::NearClient;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

pub enum ExperimentalJsonRpcMethod {
    BroadcastTxSync { tx: views::SignedTransactionView },
    Changes(near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockRequest),
    ChangesInBlock(near_jsonrpc_primitives::types::changes::RpcStateChangesRequest),
    CheckTx { tx: views::SignedTransactionView },
    GenesisConfig,
    ProtocolConfig(near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest),
    Receipt(near_jsonrpc_primitives::types::receipts::RpcReceiptRequest),
    TxStatus { tx: String },
    ValidatorsOrdered(near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest),
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
    Block(BlockReference),
    BroadcastTxAsync {
        tx: views::SignedTransactionView,
    },
    BroadcastTxCommit {
        tx: views::SignedTransactionView,
    },
    Chunk {
        id: ChunkId,
    },
    GasPrice {
        block_id: MaybeBlockId,
    },
    Health,
    LightClientProof(
        near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofRequest,
    ),
    NextLightClientBlock(
        near_jsonrpc_primitives::types::light_client::RpcLightClientNextBlockRequest,
    ),
    NetworkInfo,
    Query(near_jsonrpc_primitives::types::query::RpcQueryRequest),
    Status,
    Tx {
        hash: CryptoHash,
        id: AccountId,
    },
    Validators {
        block_id: MaybeBlockId,
    },
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

#[derive(Debug, Error)]
pub enum JsonRpcTransportSendError {
    #[error("error while serializing payload: [{0}]")]
    PayloadSerializeError(serde_json::Error),
    #[error("error while sending payload: [{0}]")]
    PayloadSendError(reqwest::Error),
}

#[derive(Debug, Error)]
pub enum JsonRpcTransportHandlerResponseError {
    #[error("error while parsing method call result: [{0}]")]
    ResultParseError(serde_json::Error),
    #[error("error while parsing method call error message: [{0}]")]
    ErrorMessageParseError(serde_json::Error),
}

#[derive(Debug, Error)]
pub enum JsonRpcTransportRecvError {
    #[error("unexpected server response: [{0:?}]")]
    UnexpectedServerResponse(Message),
    #[error("error while reading response: [{0}]")]
    PayloadRecvError(reqwest::Error),
    #[error("error while parsing server response: [{0:?}]")]
    PayloadParseError(message::Broken),
    #[error(transparent)]
    ResponseParseError(JsonRpcTransportHandlerResponseError),
}

#[derive(Debug, Error)]
pub enum RpcTransportError {
    #[error(transparent)]
    SendError(JsonRpcTransportSendError),
    #[error(transparent)]
    RecvError(JsonRpcTransportRecvError),
}

pub enum JsonRpcServerError<E> {
    RequestValidationError(RpcRequestValidationErrorKind),
    HandlerError(E),
    InternalError(serde_json::Value),
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for JsonRpcServerError<E> {}

impl<E: fmt::Display> fmt::Display for JsonRpcServerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestValidationError(err) => {
                write!(f, "request validation error: [{:?}]", err)
            }
            Self::HandlerError(err) => write!(f, "handler error: [{}]", err),
            Self::InternalError(err) => write!(f, "internal error: [{}]", err),
        }
    }
}

impl<E: fmt::Debug> fmt::Debug for JsonRpcServerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestValidationError(err) => {
                f.debug_tuple("RequestValidationError").field(err).finish()
            }
            Self::HandlerError(err) => f.debug_tuple("HandlerError").field(err).finish(),
            Self::InternalError(err) => f.debug_tuple("InternalError").field(err).finish(),
        }
    }
}

pub enum JsonRpcError<E> {
    TransportError(RpcTransportError),
    ServerError(JsonRpcServerError<E>),
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for JsonRpcError<E> {}

impl<E: fmt::Display> fmt::Display for JsonRpcError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransportError(err) => fmt::Display::fmt(err, f),
            Self::ServerError(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl<E: fmt::Debug> fmt::Debug for JsonRpcError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransportError(err) => f.debug_tuple("TransportError").field(err).finish(),
            Self::ServerError(err) => f.debug_tuple("ServerError").field(err).finish(),
        }
    }
}

pub type JsonRpcMethodCallResult<T, E> = Result<T, JsonRpcError<E>>;

impl JsonRpcMethod {
    fn method_and_params(&self) -> (&str, serde_json::Value) {
        match self {
            Block(request) => ("block", json!(request)),
            BroadcastTxAsync { tx } => ("broadcast_tx_async", json!([tx])),
            BroadcastTxCommit { tx } => ("broadcast_tx_commit", json!([tx])),
            Chunk { id } => ("chunk", json!([id])),
            GasPrice { block_id } => ("gas_price", json!([block_id])),
            Health => ("health", json!(null)),
            LightClientProof(request) => ("light_client_proof", json!(request)),
            NextLightClientBlock(request) => ("next_light_client_block", json!(request)),
            NetworkInfo => ("network_info", json!(null)),
            Query(request) => ("query", json!(request)),
            Status => ("status", json!(null)),
            Tx { hash, id } => ("tx", json!([hash, id])),
            Validators { block_id } => ("validators", json!([block_id])),
            Experimental(method) => match method {
                BroadcastTxSync { tx } => ("EXPERIMENTAL_broadcast_tx_sync", json!([tx])),
                Changes(request) => ("EXPERIMENTAL_changes", json!(request)),
                ChangesInBlock(request) => ("EXPERIMENTAL_changes_in_block", json!(request)),
                CheckTx { tx } => ("EXPERIMENTAL_check_tx", json!([tx])),
                GenesisConfig => ("EXPERIMENTAL_genesis_config", json!(null)),
                ProtocolConfig(request) => ("EXPERIMENTAL_protocol_config", json!(request)),
                Receipt(request) => ("EXPERIMENTAL_receipt", json!(request)),
                TxStatus { tx } => ("EXPERIMENTAL_tx_status", json!([tx])),
                ValidatorsOrdered(request) => ("EXPERIMENTAL_validators_ordered", json!(request)),
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

    pub async fn call_on<T: DeserializeOwned, E: DeserializeOwned>(
        &self,
        rpc_client: &NearJsonRpcClient,
    ) -> JsonRpcMethodCallResult<T, E> {
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

#[derive(Clone)]
pub struct NearJsonRpcClient {
    pub(crate) near_client: NearClient,
}

impl NearJsonRpcClient {
    pub async fn block(
        &self,
        request: BlockReference,
    ) -> JsonRpcMethodCallResult<
        views::BlockView,
        near_jsonrpc_primitives::types::blocks::RpcBlockError,
    > {
        Block(request).call_on(self).await
    }

    pub async fn broadcast_tx_async(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<CryptoHash, ()> {
        BroadcastTxAsync { tx }.call_on(self).await
    }

    pub async fn broadcast_tx_commit(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<
        views::FinalExecutionOutcomeView,
        near_jsonrpc_primitives::types::transactions::RpcTransactionError,
    > {
        BroadcastTxCommit { tx }.call_on(self).await
    }

    pub async fn chunk(
        &self,
        id: ChunkId,
    ) -> JsonRpcMethodCallResult<
        views::ChunkView,
        near_jsonrpc_primitives::types::chunks::RpcChunkError,
    > {
        Chunk { id }.call_on(self).await
    }

    pub async fn gas_price(
        &self,
        block_id: MaybeBlockId,
    ) -> JsonRpcMethodCallResult<
        views::GasPriceView,
        near_jsonrpc_primitives::types::gas_price::RpcGasPriceError,
    > {
        GasPrice { block_id }.call_on(self).await
    }

    pub async fn health(
        &self,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::status::RpcHealthResponse,
        near_jsonrpc_primitives::types::status::RpcStatusError,
    > {
        Health.call_on(self).await?;
        Ok(near_jsonrpc_primitives::types::status::RpcHealthResponse)
    }

    pub async fn light_client_proof(
        &self,
        request: near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::light_client::RpcLightClientExecutionProofResponse,
        near_jsonrpc_primitives::types::light_client::RpcLightClientProofError,
    > {
        LightClientProof(request).call_on(self).await
    }

    pub async fn next_light_client_block(
        &self,
        request: near_jsonrpc_primitives::types::light_client::RpcLightClientNextBlockRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::light_client::RpcLightClientNextBlockResponse,
        near_jsonrpc_primitives::types::light_client::RpcLightClientNextBlockError,
    > {
        NextLightClientBlock(request).call_on(self).await
    }

    pub async fn network_info(
        &self,
    ) -> JsonRpcMethodCallResult<
        near_client_primitives::types::NetworkInfoResponse,
        near_jsonrpc_primitives::types::network_info::RpcNetworkInfoError,
    > {
        NetworkInfo.call_on(self).await
    }

    pub async fn query(
        &self,
        request: near_jsonrpc_primitives::types::query::RpcQueryRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::query::RpcQueryResponse,
        near_jsonrpc_primitives::types::query::RpcQueryError,
    > {
        Query(request).call_on(self).await
    }

    pub async fn status(
        &self,
    ) -> JsonRpcMethodCallResult<
        views::StatusResponse,
        near_jsonrpc_primitives::types::status::RpcStatusError,
    > {
        Status.call_on(self).await
    }

    pub async fn tx(
        &self,
        hash: CryptoHash,
        id: AccountId,
    ) -> JsonRpcMethodCallResult<
        views::FinalExecutionOutcomeView,
        near_jsonrpc_primitives::types::transactions::RpcTransactionError,
    > {
        Tx { hash, id }.call_on(self).await
    }

    pub async fn validators(
        &self,
        block_id: MaybeBlockId,
    ) -> JsonRpcMethodCallResult<
        views::EpochValidatorInfo,
        near_jsonrpc_primitives::types::validator::RpcValidatorError,
    > {
        Validators { block_id }.call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_broadcast_tx_sync(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Experimental(BroadcastTxSync { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::changes::RpcStateChangesResponse,
        RpcError,
    > {
        Experimental(Changes(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes_in_block(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockResponse,
        RpcError,
    > {
        Experimental(ChangesInBlock(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_check_tx(
        &self,
        tx: views::SignedTransactionView,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Experimental(CheckTx { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_genesis_config(
        &self,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Experimental(GenesisConfig).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_protocol_config(
        &self,
        request: near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::config::RpcProtocolConfigResponse,
        RpcError,
    > {
        Experimental(ProtocolConfig(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_receipt(
        &self,
        request: near_jsonrpc_primitives::types::receipts::RpcReceiptRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::receipts::RpcReceiptResponse,
        RpcError,
    > {
        Experimental(Receipt(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_tx_status(
        &self,
        tx: String,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Experimental(TxStatus { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_validators_ordered(
        &self,
        request: near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest,
    ) -> JsonRpcMethodCallResult<Vec<views::validator_stake_view::ValidatorStakeView>, RpcError>
    {
        Experimental(ValidatorsOrdered(request)).call_on(self).await
    }

    #[cfg(feature = "sandbox")]
    pub async fn sandbox_patch_state(
        &self,
        request: near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateResponse,
        RpcError,
    > {
        Sandbox(PatchState(request)).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_set_weight(
        &self,
        height: u64,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Adversarial(SetWeight(height)).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_disable_header_sync(
        &self,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Adversarial(DisableHeaderSync).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_disable_doomslug(
        &self,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Adversarial(DisableDoomslug).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_produce_blocks(
        &self,
        num_blocks: u64,
        only_valid: bool,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
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
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Adversarial(SwitchToHeight(height)).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_get_saved_blocks(
        &self,
    ) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Adversarial(GetSavedBlocks).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_check_store(&self) -> JsonRpcMethodCallResult<serde_json::Value, RpcError> {
        Adversarial(CheckStore).call_on(self).await
    }
}
