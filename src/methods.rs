use std::io;

use serde_json::json;

mod chk {
    // this lets us make the RpcMethod trait public but non-implementable by users outside this crate
    pub trait ValidRpcMethod {}
}

pub trait RpcMethod: chk::ValidRpcMethod
where
    Self::Result: serde::de::DeserializeOwned,
    Self::Error: serde::de::DeserializeOwned,
{
    type Result;
    type Error;

    const METHOD_NAME: &'static str;

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(serde_json::json!(null))
    }

    fn parse_result(value: serde_json::Value) -> Result<Self::Result, serde_json::Error> {
        serde_json::from_value(value)
    }
}

macro_rules! impl_method {
    (@main
        $method_name:ident: {
            $(#![$meta:meta])*
            $(exports: { $($exports:tt)+ })?

            import_scope: $import_scope:expr;

            let constructor = $constructor_str:expr;
            let constructor_import = $constructor_import_str:expr;
            let constructor_call = $constructor_call_str:expr;
            let result = $result_str:expr;
            let result_import = $result_import_str:expr;
            let error = $error_str:expr;
            let error_import = $error_import_str:expr;

            impl RpcMethod for $request_ty:ty {
                type Result = $result_ty:ty;
                type Error = $error_ty:ty;

                $(params(&$this:ident) $param_exec:block)?
                $(parse_result($value:ident) $result_parser:block)?
            }
        }
    ) => {
        #[allow(non_snake_case)]
        pub mod $method_name {
            $(#![$meta])*
            #![doc = ""]
            //! # Example
            //!
            //! ```ignore
            //! use near_jsonrpc_client::{methods, JsonRpcClient, JsonRpcMethodCallResult};
            //!
            #![doc = $import_scope]
            #![doc = $constructor_import_str]
            #![doc = $result_import_str]
            #![doc = $error_import_str]
            //! };
            //!
            //! let mainnet_client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
            //!
            #![doc = $constructor_call_str]
            //!
            //! let result: JsonRpcMethodCallResult<
            #![doc = $result_import_str]
            #![doc = $error_import_str]
            //! > = mainnet_client.call(&request);
            //! ```
            //!
            //! - RPC Method Constructor: [
            #![doc = $constructor_str]
            //! ]
            //! - RPC Method Response Type: [
            #![doc = $result_str]
            //! ]
            //! - RPC Method Error Type: [
            #![doc = $error_str]
            //! ]

            use super::*;

            $($($exports)+)?

            impl chk::ValidRpcMethod for $request_ty {}

            impl RpcMethod for $request_ty {
                type Result = $result_ty;
                type Error = $error_ty;

                const METHOD_NAME: &'static str = stringify!($method_name);

                $(
                    fn params(&$this) -> Result<serde_json::Value, io::Error> {
                        Ok($param_exec)
                    }
                )?

                $(
                    fn parse_result($value: serde_json::Value) -> Result<Self::Result, serde_json::Error> {
                        Ok($result_parser)
                    }
                )?
            }
        }
    };
    (
        $method_name:ident: {
            $(#![$meta:meta])*
            $(exports: { $($exports:tt)+ })?

            impl RpcMethod for $request_ty:ty {
                type Result = $result_ty:ty;
                type Error = $error_ty:ty;

                $(params(&$this:ident) $param_exec:block)?
                $(parse_result($value:ident) $result_parser:block)?
            }
        }
    ) => {
        impl_method!(@main $method_name: {
            $(#![$meta])*
            $(exports: { $($exports)+ })?

            import_scope: concat!("use methods::", stringify!($method_name), "::{");

            let constructor = stringify!($request_ty);
            let constructor_import = concat!("    ", stringify!($request_ty), ",");
            let constructor_call = concat!("let request = ", stringify!($request_ty), " {...}; // <-- create a valid request here");
            let result = stringify!($result_ty);
            let result_import = concat!("    ", stringify!($result_ty), ",");
            let error = stringify!($error_ty);
            let error_import = concat!("    ", stringify!($error_ty), ",");

            impl RpcMethod for $request_ty {
                type Result = $result_ty;
                type Error = $error_ty;

                $(params(&$this) $param_exec)?
                $(parse_result($value) $result_parser)?
            }
        });
    };
}

impl_method! {
    block: {
        exports: {
            pub use near_jsonrpc_primitives::types::blocks::{RpcBlockError, RpcBlockRequest};
            pub use near_primitives::views::BlockView;
        }

        impl RpcMethod for RpcBlockRequest {
            type Result = BlockView;
            type Error = RpcBlockError;

            params(&self) { json!([self]) }
        }
    }
}

fn serialize_signed_transaction(
    tx: &near_primitives::transaction::SignedTransaction,
) -> Result<String, io::Error> {
    Ok(near_primitives::serialize::to_base64(
        &borsh::BorshSerialize::try_to_vec(&tx)?,
    ))
}

impl_method! {
    broadcast_tx_async: {
        exports: {
            pub use near_primitives::hash::CryptoHash;
            pub use near_primitives::transaction::SignedTransaction;

            #[derive(Debug)]
            pub struct RpcBroadcastTxAsyncRequest {
                pub signed_transaction: SignedTransaction,
            }

            impl From<RpcBroadcastTxAsyncRequest>
                for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
            {
                fn from(this: RpcBroadcastTxAsyncRequest) -> Self {
                    Self {
                        signed_transaction: this.signed_transaction,
                    }
                }
            }

            pub type RpcBroadcastTxAsyncError = ();
        }

        impl RpcMethod for RpcBroadcastTxAsyncRequest {
            type Result = CryptoHash;
            type Error = RpcBroadcastTxAsyncError;

            params(&self) {
                json!([serialize_signed_transaction(&self.signed_transaction)?])
            }
        }
    }
}

impl_method! {
    broadcast_tx_commit: {
        exports: {
            pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
            pub use near_primitives::transaction::SignedTransaction;
            pub use near_primitives::views::FinalExecutionOutcomeView;

            #[derive(Debug)]
            pub struct RpcBroadcastTxCommitRequest {
                pub signed_transaction: SignedTransaction,
            }

            impl From<RpcBroadcastTxCommitRequest>
                for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
            {
                fn from(this: RpcBroadcastTxCommitRequest) -> Self {
                    Self {
                        signed_transaction: this.signed_transaction,
                    }
                }
            }
        }

        impl RpcMethod for RpcBroadcastTxCommitRequest {
            type Result = FinalExecutionOutcomeView;
            type Error = RpcTransactionError;

            params(&self) {
                json!([serialize_signed_transaction(&self.signed_transaction)?])
            }
        }
    }
}

impl_method! {
    chunk: {
        exports: {
            pub use near_jsonrpc_primitives::types::chunks::{RpcChunkError, RpcChunkRequest};
            pub use near_primitives::views::ChunkView;
        }

        impl RpcMethod for RpcChunkRequest {
            type Result = ChunkView;
            type Error = RpcChunkError;

            params(&self) { json!([self]) }
        }
    }
}

impl_method! {
    gas_price: {
        exports: {
            pub use near_jsonrpc_primitives::types::gas_price::{
                RpcGasPriceError, RpcGasPriceRequest,
            };
            pub use near_primitives::views::GasPriceView;
        }

        impl RpcMethod for RpcGasPriceRequest {
            type Result = GasPriceView;
            type Error = RpcGasPriceError;

            params(&self) { json!([self]) }
        }
    }
}

impl_method! {
    health: {
        exports: {
            pub use near_jsonrpc_primitives::types::status::{
                RpcHealthResponse, RpcStatusResponse,
            };

            #[derive(Debug)]
            pub struct RpcHealthRequest;
        }

        impl RpcMethod for RpcHealthRequest {
            type Result = RpcHealthResponse;
            type Error = RpcStatusResponse;
        }
    }
}

impl_method! {
    light_client_proof: {
        exports: {
            pub use near_jsonrpc_primitives::types::light_client::{
                RpcLightClientExecutionProofRequest, RpcLightClientExecutionProofResponse,
                RpcLightClientProofError,
            };
        }

        impl RpcMethod for RpcLightClientExecutionProofRequest {
            type Result = RpcLightClientExecutionProofResponse;
            type Error = RpcLightClientProofError;

            params(&self) { json!(self) }
        }
    }
}

impl_method! {
    next_light_client_block: {
        exports: {
            pub use near_jsonrpc_primitives::types::light_client::{
                RpcLightClientNextBlockError, RpcLightClientNextBlockRequest,
            };
            pub use near_primitives::views::LightClientBlockView;
        }

        impl RpcMethod for RpcLightClientNextBlockRequest {
            type Result = Option<LightClientBlockView>;
            type Error = RpcLightClientNextBlockError;

            params(&self) { json!(self) }
        }
    }
}

impl_method! {
    network_info: {
        exports: {
            pub use near_client_primitives::types::NetworkInfoResponse;
            pub use near_jsonrpc_primitives::types::network_info::RpcNetworkInfoError;

            #[derive(Debug)]
            pub struct RpcNetworkInfoRequest;
        }

        impl RpcMethod for RpcNetworkInfoRequest {
            type Result = NetworkInfoResponse;
            type Error = RpcNetworkInfoError;
        }
    }
}

impl_method! {
    query: {
        exports: {
            pub use near_jsonrpc_primitives::types::query::{
                RpcQueryError, RpcQueryRequest, RpcQueryResponse,
            };
        }

        impl RpcMethod for RpcQueryRequest {
            type Result = RpcQueryResponse;
            type Error = RpcQueryError;

            params(&self) { json!(self) }
        }
    }
}

impl_method! {
    status: {
        exports: {
            pub use near_jsonrpc_primitives::types::status::RpcStatusError;
            pub use near_primitives::views::StatusResponse;

            #[derive(Debug)]
            pub struct RpcStatusRequest;
        }

        impl RpcMethod for RpcStatusRequest {
            type Result = StatusResponse;
            type Error = RpcStatusError;
        }
    }
}

impl_method! {
    tx: {
        exports: {
            use near_jsonrpc_primitives::types::transactions::TransactionInfo;
            pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
            pub use near_primitives::views::FinalExecutionOutcomeViewEnum;
            pub type RpcTransactionStatusRequest = near_jsonrpc_primitives::types::transactions::RpcTransactionStatusCommonRequest;
        }

        impl RpcMethod for RpcTransactionStatusRequest {
            type Result = FinalExecutionOutcomeViewEnum;
            type Error = RpcTransactionError;

            params(&self) {
                match &self.transaction_info {
                    TransactionInfo::Transaction(signed_transaction) => {
                        json!([serialize_signed_transaction(&signed_transaction)?])
                    }
                    TransactionInfo::TransactionId { hash, account_id } => {
                        json!([hash, account_id])
                    }
                }
            }
        }
    }
}

impl_method! {
    validators: {
        exports: {
            pub use near_jsonrpc_primitives::types::validator::{
                RpcValidatorError, RpcValidatorRequest,
            };
            pub use near_primitives::views::EpochValidatorInfo;
        }

        impl RpcMethod for RpcValidatorRequest {
            type Result = EpochValidatorInfo;
            type Error = RpcValidatorError;

            params(&self) { json!([self]) }
        }
    }
}

impl_method! {
    EXPERIMENTAL_broadcast_tx_sync: {
        exports: {
            pub use near_jsonrpc_primitives::types::transactions::{
                RpcBroadcastTxSyncResponse, RpcTransactionError,
            };
            pub use near_primitives::transaction::SignedTransaction;

            #[derive(Debug)]
            pub struct RpcBroadcastTxSyncRequest {
                pub signed_transaction: SignedTransaction,
            }

            impl From<RpcBroadcastTxSyncRequest>
                for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
            {
                fn from(this: RpcBroadcastTxSyncRequest) -> Self {
                    Self { signed_transaction: this.signed_transaction }
                }
            }
        }

        impl RpcMethod for RpcBroadcastTxSyncRequest {
            type Result = RpcBroadcastTxSyncResponse;
            type Error = RpcTransactionError;

            params(&self) {
                json!([serialize_signed_transaction(&self.signed_transaction)?])
            }
        }
    }
}

impl_method! {
    EXPERIMENTAL_changes: {
        exports: {
            pub use near_jsonrpc_primitives::types::changes::{
                RpcStateChangesError, RpcStateChangesInBlockByTypeRequest,
                RpcStateChangesInBlockResponse,
            };
        }

        impl RpcMethod for RpcStateChangesInBlockByTypeRequest {
            type Result = RpcStateChangesInBlockResponse;
            type Error = RpcStateChangesError;

            params(&self) { json!(self) }
        }
    }
}

impl_method! {
    EXPERIMENTAL_changes_in_block: {
        exports: {
            pub use near_jsonrpc_primitives::types::changes::{
                RpcStateChangesError, RpcStateChangesInBlockRequest,
                RpcStateChangesInBlockByTypeResponse,
            };
        }

        impl RpcMethod for RpcStateChangesInBlockRequest {
            type Result = RpcStateChangesInBlockByTypeResponse;
            type Error = RpcStateChangesError;

            params(&self) { json!(self) }
        }
    }
}

impl_method! {
    EXPERIMENTAL_check_tx: {
        exports: {
            pub use near_jsonrpc_primitives::types::transactions::{
                RpcBroadcastTxSyncResponse, RpcTransactionError,
            };
            pub use near_primitives::transaction::SignedTransaction;

            #[derive(Debug)]
            pub struct RpcCheckTxRequest {
                pub signed_transaction: SignedTransaction,
            }

            impl From<RpcCheckTxRequest>
                for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
            {
                fn from(this: RpcCheckTxRequest) -> Self {
                    Self { signed_transaction: this.signed_transaction }
                }
            }
        }

        impl RpcMethod for RpcCheckTxRequest {
            type Result = RpcBroadcastTxSyncResponse;
            type Error = RpcTransactionError;

            params(&self) {
                json!([serialize_signed_transaction(&self.signed_transaction)?])
            }
        }
    }
}

impl_method! {
    EXPERIMENTAL_genesis_config: {
        exports: {
            pub use near_chain_configs::GenesisConfig;

            #[derive(Debug)]
            pub struct RpcGenesisConfigRequest;

            pub type RpcGenesisConfigError = ();
        }

        impl RpcMethod for RpcGenesisConfigRequest {
            type Result = GenesisConfig;
            type Error = RpcGenesisConfigError;
        }
    }
}

impl_method! {
    EXPERIMENTAL_protocol_config: {
        exports: {
            pub use near_chain_configs::ProtocolConfigView;
            pub use near_jsonrpc_primitives::types::config::{
                RpcProtocolConfigError, RpcProtocolConfigRequest,
            };
        }

        impl RpcMethod for RpcProtocolConfigRequest {
            type Result = ProtocolConfigView;
            type Error = RpcProtocolConfigError;

            params(&self) { json!(self) }
        }
    }
}

impl_method! {
    EXPERIMENTAL_receipt: {
        exports: {
            pub use near_jsonrpc_primitives::types::receipts::{
                RpcReceiptError, RpcReceiptRequest,
            };
            pub use near_primitives::views::ReceiptView;
        }

        impl RpcMethod for RpcReceiptRequest {
            type Result = ReceiptView;
            type Error = RpcReceiptError;

            params(&self) { json!(self) }
        }
    }
}

impl_method! {
    EXPERIMENTAL_validators_ordered: {
        exports: {
            pub use near_jsonrpc_primitives::types::validator::{
                RpcValidatorError, RpcValidatorsOrderedRequest, RpcValidatorsOrderedResponse,
            };
        }

        impl RpcMethod for RpcValidatorsOrderedRequest {
            type Result = RpcValidatorsOrderedResponse;
            type Error = RpcValidatorError;

            params(&self) { json!(self) }
        }
    }
}

#[cfg(feature = "sandbox")]
impl_method! {
    sandbox_patch_state: {
        exports: {
            pub use near_jsonrpc_primitives::types::sandbox::{
                RpcSandboxPatchStateError, RpcSandboxPatchStateRequest,
                RpcSandboxPatchStateResponse,
            };
        }

        impl RpcMethod for RpcSandboxPatchStateRequest {
            type Result = RpcSandboxPatchStateResponse;
            type Error = RpcSandboxPatchStateError;

            params(&self) { json!(self) }
        }
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    adv_set_weight: {
        exports: {
            #[derive(Debug)]
            pub struct RpcAdversarialSetWeightRequest { pub height: u64 }

            pub type RpcAdversarialSetWeightResponse = ();
            pub type RpcAdversarialSetWeightError = ();
        }

        impl RpcMethod for RpcAdversarialSetWeightRequest {
            type Result = RpcAdversarialSetWeightResponse;
            type Error = RpcAdversarialSetWeightError;

            params(&self) { json!(self.height) }
            parse_result(value) {
                serde_json::from_value(value)?;
                ()
            }
        }
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    adv_disable_header_sync: {
        exports: {
            #[derive(Debug)]
            pub struct RpcAdversarialDisableHeaderSyncRequest;

            pub type RpcAdversarialDisableHeaderSyncResponse = ();
            pub type RpcAdversarialDisableHeaderSyncError = ();
        }

        impl RpcMethod for RpcAdversarialDisableHeaderSyncRequest {
            type Result = RpcAdversarialDisableHeaderSyncResponse;
            type Error = RpcAdversarialDisableHeaderSyncError;

            parse_result(value) {
                serde_json::from_value(value)?;
                ()
            }
        }
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    adv_disable_doomslug: {
        exports: {
            #[derive(Debug)]
            pub struct RpcAdversarialDisableDoomslugRequest;

            pub type RpcAdversarialDisableDoomslugResponse = ();
            pub type RpcAdversarialDisableDoomslugError = ();
        }

        impl RpcMethod for RpcAdversarialDisableDoomslugRequest {
            type Result = RpcAdversarialDisableDoomslugResponse;
            type Error = RpcAdversarialDisableDoomslugError;

            parse_result(value) {
                serde_json::from_value(value)?;
                ()
            }
        }
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    adv_produce_blocks: {
        exports: {
            #[derive(Debug)]
            pub struct RpcAdversarialProduceBlocksRequest {
                pub num_blocks: u64,
                pub only_valid: bool,
            }

            pub type RpcAdversarialProduceBlocksResponse = ();
            pub type RpcAdversarialProduceBlocksError = ();
        }

        impl RpcMethod for RpcAdversarialProduceBlocksRequest {
            type Result = RpcAdversarialProduceBlocksResponse;
            type Error = RpcAdversarialProduceBlocksError;

            params(&self) { json!([self.num_blocks, self.only_valid]) }
            parse_result(value) {
                serde_json::from_value(value)?;
                ()
            }
        }
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    adv_switch_to_height: {
        exports: {
            #[derive(Debug)]
            pub struct RpcAdversarialSwitchToHeightRequest { pub height: u64 }

            pub type RpcAdversarialSwitchToHeightResponse = ();
            pub type RpcAdversarialSwitchToHeightError = ();
        }

        impl RpcMethod for RpcAdversarialSwitchToHeightRequest {
            type Result = RpcAdversarialSwitchToHeightResponse;
            type Error = RpcAdversarialSwitchToHeightError;

            params(&self) { json!([self.height]) }
            parse_result(value) {
                serde_json::from_value(value)?;
                ()
            }
        }
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    adv_get_saved_blocks: {
        exports: {
            #[derive(Debug)]
            pub struct RpcAdversarialGetSavedBlocksRequest;

            pub type RpcAdversarialGetSavedBlocksResponse = u64;
            pub type RpcAdversarialGetSavedBlocksError = ();
        }

        impl RpcMethod for RpcAdversarialGetSavedBlocksRequest {
            type Result = RpcAdversarialGetSavedBlocksResponse;
            type Error = RpcAdversarialGetSavedBlocksError;
        }
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    adv_check_store: {
        exports: {
            #[derive(Debug)]
            pub struct RpcAdversarialCheckStoreRequest;

            pub type RpcAdversarialCheckStoreResponse = u64;
            pub type RpcAdversarialCheckStoreError = ();
        }

        impl RpcMethod for RpcAdversarialCheckStoreRequest {
            type Result = RpcAdversarialCheckStoreResponse;
            type Error = RpcAdversarialCheckStoreError;
        }
    }
}
