use std::io;

use serde_json::json;

mod chk {
    /// this lets us make the RpcMethod trait public but non-implementable by it's users outside this crate
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
}

macro_rules! impl_method {
    (
        $method_name:ident: {
            $(exports: { $($exports:tt)+ })?

            impl RpcMethod for $request_ty:ty {
                type $type_variant_1:ident = $variant_1_ty:ty;
                type $type_variant_2:ident = $variant_2_ty:ty;

                $(params(&$this:ident) { $param_exec:expr })?
            }
        }
    ) => {
        pub mod $method_name {
            use super::*;

            $($($exports)+)?

            impl chk::ValidRpcMethod for $request_ty {}

            impl RpcMethod for $request_ty {
                type $type_variant_1 = $variant_1_ty;
                type $type_variant_2 = $variant_2_ty;

                const METHOD_NAME: &'static str = stringify!($method_name);

                $(
                    fn params(&$this) -> Result<serde_json::Value, io::Error> {
                        Ok($param_exec)
                    }
                )?
            }
        }
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
            pub struct RpcTxAsyncRequest {
                pub signed_transaction: SignedTransaction,
            }

            impl From<RpcTxAsyncRequest>
                for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
            {
                fn from(this: RpcTxAsyncRequest) -> Self {
                    Self {
                        signed_transaction: this.signed_transaction,
                    }
                }
            }
        }

        impl RpcMethod for RpcTxAsyncRequest {
            type Result = CryptoHash;
            type Error = ();

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
            pub struct RpcTxCommitRequest {
                pub signed_transaction: SignedTransaction,
            }

            impl From<RpcTxCommitRequest>
                for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
            {
                fn from(this: RpcTxCommitRequest) -> Self {
                    Self {
                        signed_transaction: this.signed_transaction,
                    }
                }
            }
        }

        impl RpcMethod for RpcTxCommitRequest {
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
            pub use near_jsonrpc_primitives::types::transactions::{
                RpcTransactionError, RpcTransactionStatusCommonRequest,
            };
            pub use near_primitives::views::FinalExecutionOutcomeView;
        }

        impl RpcMethod for RpcTransactionStatusCommonRequest {
            type Result = FinalExecutionOutcomeView;
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
