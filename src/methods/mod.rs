//! This module contains all the RPC methods.
use std::io;

use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

mod private {
    pub trait Sealed {}
}

/// A trait identifying valid NEAR JSON-RPC methods.
pub trait RpcMethod: private::Sealed
where
    Self::Response: RpcHandlerResponse,
    Self::Error: RpcHandlerError,
{
    type Response;
    type Error;

    fn method_name(&self) -> &str;

    fn params(&self) -> Result<serde_json::Value, io::Error>;

    fn parse_handler_response(
        response: serde_json::Value,
    ) -> Result<Result<Self::Response, Self::Error>, serde_json::Error> {
        Self::Response::parse(response).map(Ok)
    }
}

impl<T> private::Sealed for &T where T: private::Sealed {}
impl<T> RpcMethod for &T
where
    T: RpcMethod,
{
    type Response = T::Response;
    type Error = T::Error;

    fn method_name(&self) -> &str {
        T::method_name(self)
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        T::params(self)
    }

    fn parse_handler_response(
        response: serde_json::Value,
    ) -> Result<Result<Self::Response, Self::Error>, serde_json::Error> {
        T::parse_handler_response(response)
    }
}

/// A trait identifying valid NEAR JSON-RPC method responses.
pub trait RpcHandlerResponse: serde::de::DeserializeOwned {
    fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

/// A trait identifying valid NEAR JSON-RPC errors.
pub trait RpcHandlerError: serde::de::DeserializeOwned {
    /// Parser for the `.error_struct` field in RpcError.
    fn parse(handler_error: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(handler_error)
    }

    /// Parser for the `.data` field in RpcError, not `.error_struct`.
    ///
    /// This would only ever be used as a fallback if [`RpcHandlerError::parse`] fails.
    ///
    /// Defaults to `None` meaning there's no alternative deserialization available.
    fn parse_legacy_error(_error: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
        None
    }
}

pub mod block;
pub mod broadcast_tx_async;
pub mod broadcast_tx_commit;
pub mod chunk;
pub mod gas_price;
pub mod health;
pub mod light_client_proof;
pub mod network_info;
pub mod next_light_client_block;
pub mod query;
pub mod send_tx;
pub mod status;
pub mod tx;
pub mod validators;

// ======== experimental ========
mod experimental;
pub use experimental::EXPERIMENTAL_changes;
pub use experimental::EXPERIMENTAL_changes_in_block;
pub use experimental::EXPERIMENTAL_genesis_config;
pub use experimental::EXPERIMENTAL_protocol_config;
pub use experimental::EXPERIMENTAL_receipt;
pub use experimental::EXPERIMENTAL_tx_status;
pub use experimental::EXPERIMENTAL_validators_ordered;
// ======== experimental ========

// ======== any ========
#[cfg(feature = "any")]
mod any;
#[cfg(feature = "any")]
pub use any::{request as any, RpcAnyRequest};
// ======== any ========

// ======== sandbox ========
#[cfg(feature = "sandbox")]
mod sandbox;

#[cfg(feature = "sandbox")]
pub use sandbox::sandbox_patch_state;

#[cfg(feature = "sandbox")]
pub use sandbox::sandbox_fast_forward;
// ======== sandbox ========

// ======== adversarial ========
#[cfg(feature = "adversarial")]
mod adversarial;

#[cfg(feature = "adversarial")]
pub use adversarial::adv_set_weight;

#[cfg(feature = "adversarial")]
pub use adversarial::adv_disable_header_sync;

#[cfg(feature = "adversarial")]
pub use adversarial::adv_disable_doomslug;

#[cfg(feature = "adversarial")]
pub use adversarial::adv_produce_blocks;

#[cfg(feature = "adversarial")]
pub use adversarial::adv_switch_to_height;

#[cfg(feature = "adversarial")]
pub use adversarial::adv_get_saved_blocks;

#[cfg(feature = "adversarial")]
pub use adversarial::adv_check_store;
// ======== adversarial ========

/// Converts an RPC Method into JSON.
pub fn to_json<M: RpcMethod>(method: &M) -> Result<serde_json::Value, io::Error> {
    let request_payload = near_jsonrpc_primitives::message::Message::request(
        method.method_name().to_string(),
        method.params()?,
    );

    Ok(json!(request_payload))
}

mod common {
    use super::*;

    // workaround for deserializing partially serialized
    // error types missing the `error_message` field in
    // their UnknownBlock variants.
    macro_rules! _parse_unknown_block {
        ($json:expr => $err_ty:ident) => {
            match $json {
                err => {
                    if err["name"] == "UNKNOWN_BLOCK" {
                        Ok($err_ty::UnknownBlock {
                            error_message: "".to_string(),
                        })
                    } else {
                        serde_json::from_value(err)
                    }
                }
            }
        };
    }
    pub(crate) use _parse_unknown_block as parse_unknown_block;

    pub fn serialize_signed_transaction(
        tx: &near_primitives::transaction::SignedTransaction,
    ) -> Result<String, io::Error> {
        Ok(near_primitives::serialize::to_base64(&borsh::to_vec(&tx)?))
    }

    // adv_*
    #[cfg(feature = "adversarial")]
    impl RpcHandlerError for () {}

    // adv_*
    #[cfg(feature = "adversarial")]
    impl RpcHandlerResponse for () {
        fn parse(_value: serde_json::Value) -> Result<Self, serde_json::Error> {
            Ok(())
        }
    }

    #[cfg(feature = "any")]
    impl RpcHandlerResponse for serde_json::Value {
        fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
            Ok(value)
        }
    }

    #[cfg(feature = "any")]
    impl RpcHandlerError for serde_json::Value {
        fn parse(handler_error: serde_json::Value) -> Result<Self, serde_json::Error> {
            Ok(handler_error)
        }
    }

    // broadcast_tx_commit, tx
    impl RpcHandlerResponse for near_primitives::views::FinalExecutionOutcomeView {}

    // broadcast_tx_commit, tx, EXPERIMENTAL_tx_status
    impl RpcHandlerError for near_jsonrpc_primitives::types::transactions::RpcTransactionError {
        fn parse_legacy_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
            match serde_json::from_value::<near_jsonrpc_primitives::errors::ServerError>(value) {
                Ok(near_jsonrpc_primitives::errors::ServerError::TxExecutionError(
                    near_primitives::errors::TxExecutionError::InvalidTxError(context),
                )) => Some(Ok(Self::InvalidTransaction { context })),
                Err(err) => Some(Err(err)),
                _ => None,
            }
        }
    }

    // health, status
    impl RpcHandlerError for near_jsonrpc_primitives::types::status::RpcStatusError {}

    // EXPERIMENTAL_changes, EXPERIMENTAL_changes_in_block
    impl RpcHandlerError for near_jsonrpc_primitives::types::changes::RpcStateChangesError {
        fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
            parse_unknown_block!(value => Self)
        }
    }

    // send_tx
    impl RpcHandlerResponse for near_jsonrpc_primitives::types::transactions::RpcTransactionResponse {}

    // validators, EXPERIMENTAL_validators_ordered
    impl RpcHandlerError for near_jsonrpc_primitives::types::validator::RpcValidatorError {}
}
