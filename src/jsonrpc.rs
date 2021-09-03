//! RPC API Client for the NEAR Protocol
#![deprecated(note = "deprecacted in favor of NearClient::call() the and RpcMethod trait")]

use std::io;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind};
use near_jsonrpc_primitives::message::{from_slice, Message};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockId, BlockReference, MaybeBlockId, ShardId};
use near_primitives::views;

use super::{errors::*, NearClient};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

pub enum ExperimentalJsonRpcMethod {
    BroadcastTxSync {
        tx: near_primitives::transaction::SignedTransaction,
    },
    Changes(near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockByTypeRequest),
    ChangesInBlock(BlockReference),
    CheckTx {
        tx: near_primitives::transaction::SignedTransaction,
    },
    GenesisConfig,
    ProtocolConfig(BlockReference),
    Receipt(near_jsonrpc_primitives::types::receipts::ReceiptReference),
    TxStatus {
        tx: near_primitives::transaction::SignedTransaction,
    },
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
        tx: near_primitives::transaction::SignedTransaction,
    },
    BroadcastTxCommit {
        tx: near_primitives::transaction::SignedTransaction,
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

pub type JsonRpcMethodCallResult<T, E> = Result<T, JsonRpcError<E>>;

fn serialize_signed_transaction(
    tx: &near_primitives::transaction::SignedTransaction,
) -> Result<String, io::Error> {
    Ok(near_primitives::serialize::to_base64(
        &borsh::BorshSerialize::try_to_vec(&tx)?,
    ))
}

impl JsonRpcMethod {
    fn method_and_params(&self) -> Result<(&str, serde_json::Value), io::Error> {
        let result = match self {
            Block(request) => ("block", json!(request)),
            BroadcastTxAsync { tx } => (
                "broadcast_tx_async",
                json!([serialize_signed_transaction(tx)?]),
            ),
            BroadcastTxCommit { tx } => (
                "broadcast_tx_commit",
                json!([serialize_signed_transaction(tx)?]),
            ),
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
                BroadcastTxSync { tx } => (
                    "EXPERIMENTAL_broadcast_tx_sync",
                    json!([serialize_signed_transaction(tx)?]),
                ),
                Changes(request) => ("EXPERIMENTAL_changes", json!(request)),
                ChangesInBlock(request) => ("EXPERIMENTAL_changes_in_block", json!(request)),
                CheckTx { tx } => (
                    "EXPERIMENTAL_check_tx",
                    json!([serialize_signed_transaction(tx)?]),
                ),
                GenesisConfig => ("EXPERIMENTAL_genesis_config", json!(null)),
                ProtocolConfig(request) => ("EXPERIMENTAL_protocol_config", json!(request)),
                Receipt(request) => ("EXPERIMENTAL_receipt", json!(request)),
                TxStatus { tx } => (
                    "EXPERIMENTAL_tx_status",
                    json!([serialize_signed_transaction(tx)?]),
                ),
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
        };
        Ok(result)
    }

    pub async fn call_on<T: DeserializeOwned, E: DeserializeOwned>(
        &self,
        rpc_client: &NearJsonRpcClient,
    ) -> JsonRpcMethodCallResult<T, E> {
        let (method_name, params) = self.method_and_params().map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSerializeError(err),
            ))
        })?;
        let request_payload = Message::request(method_name.to_string(), Some(params));
        let request_payload = serde_json::to_vec(&request_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSerializeError(err.into()),
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
                    loop {
                        if let RpcError { data: Some(err), .. } = err {
                            if let Ok(info) = serde_json::from_value::<String>(err) {
                                break RpcError::new_internal_error(None, info);
                            };
                        };
                        break RpcError::new_internal_error(None, format!("<no data>"));
                    }
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
        tx: near_primitives::transaction::SignedTransaction,
    ) -> JsonRpcMethodCallResult<CryptoHash, ()> {
        BroadcastTxAsync { tx }.call_on(self).await
    }

    pub async fn broadcast_tx_commit(
        &self,
        tx: near_primitives::transaction::SignedTransaction,
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
        Option<near_primitives::views::LightClientBlockView>,
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
        tx: near_primitives::transaction::SignedTransaction,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::transactions::RpcBroadcastTxSyncResponse,
        near_jsonrpc_primitives::types::transactions::RpcTransactionError,
    > {
        Experimental(BroadcastTxSync { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockByTypeRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockByTypeResponse,
        near_jsonrpc_primitives::types::changes::RpcStateChangesError,
    > {
        Experimental(Changes(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes_in_block(
        &self,
        request: BlockReference,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockResponse,
        near_jsonrpc_primitives::types::changes::RpcStateChangesError,
    > {
        Experimental(ChangesInBlock(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_check_tx(
        &self,
        tx: near_primitives::transaction::SignedTransaction,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::transactions::RpcBroadcastTxSyncResponse,
        near_jsonrpc_primitives::types::transactions::RpcTransactionError,
    > {
        Experimental(CheckTx { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_genesis_config(
        &self,
    ) -> JsonRpcMethodCallResult<near_chain_configs::GenesisConfig, ()> {
        Experimental(GenesisConfig).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_protocol_config(
        &self,
        request: BlockReference,
    ) -> JsonRpcMethodCallResult<
        near_chain_configs::ProtocolConfigView,
        near_jsonrpc_primitives::types::config::RpcProtocolConfigError,
    > {
        Experimental(ProtocolConfig(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_receipt(
        &self,
        request: near_jsonrpc_primitives::types::receipts::ReceiptReference,
    ) -> JsonRpcMethodCallResult<
        near_primitives::views::ReceiptView,
        near_jsonrpc_primitives::types::receipts::RpcReceiptError,
    > {
        Experimental(Receipt(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_tx_status(
        &self,
        tx: near_primitives::transaction::SignedTransaction,
    ) -> JsonRpcMethodCallResult<
        views::FinalExecutionOutcomeWithReceiptView,
        near_jsonrpc_primitives::types::transactions::RpcTransactionError,
    > {
        Experimental(TxStatus { tx }).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_validators_ordered(
        &self,
        request: near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedResponse,
        near_jsonrpc_primitives::types::validator::RpcValidatorError,
    > {
        Experimental(ValidatorsOrdered(request)).call_on(self).await
    }

    #[cfg(feature = "sandbox")]
    pub async fn sandbox_patch_state(
        &self,
        request: near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateRequest,
    ) -> JsonRpcMethodCallResult<
        near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateResponse,
        near_jsonrpc_primitives::types::sandbox::RpcSandboxPatchStateError,
    > {
        Sandbox(PatchState(request)).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_set_weight(&self, height: u64) -> JsonRpcMethodCallResult<(), ()> {
        Adversarial(SetWeight(height)).call_on(self).await?;
        Ok(())
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_disable_header_sync(&self) -> JsonRpcMethodCallResult<(), ()> {
        Adversarial(DisableHeaderSync).call_on(self).await?;
        Ok(())
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_disable_doomslug(&self) -> JsonRpcMethodCallResult<(), ()> {
        Adversarial(DisableDoomslug).call_on(self).await?;
        Ok(())
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_produce_blocks(
        &self,
        num_blocks: u64,
        only_valid: bool,
    ) -> JsonRpcMethodCallResult<(), ()> {
        Adversarial(ProduceBlocks {
            num_blocks,
            only_valid,
        })
        .call_on(self)
        .await?;
        Ok(())
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_switch_to_height(&self, height: u64) -> JsonRpcMethodCallResult<(), ()> {
        Adversarial(SwitchToHeight(height)).call_on(self).await?;
        Ok(())
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_get_saved_blocks(&self) -> JsonRpcMethodCallResult<u64, ()> {
        Adversarial(GetSavedBlocks).call_on(self).await
    }

    #[cfg(feature = "adversarial")]
    pub async fn adv_check_store(&self) -> JsonRpcMethodCallResult<u64, ()> {
        Adversarial(CheckStore).call_on(self).await
    }
}
