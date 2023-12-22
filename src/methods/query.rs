//! This module allows you to make generic requests to the network.
//!
//! The `RpcQueryRequest` struct takes in a [`BlockReference`](https://docs.rs/near-primitives/0.12.0/near_primitives/types/enum.BlockReference.html) and a [`QueryRequest`](https://docs.rs/near-primitives/0.12.0/near_primitives/views/enum.QueryRequest.html).
//!
//! The `BlockReference` enum allows you to specify a block by `Finality`, `BlockId` or `SyncCheckpoint`.
//!
//! The `QueryRequest` enum provides multiple variaints for performing the following actions:
//! - View an account's details
//! - View a contract's code
//! - View the state of an account
//! - View the `AccessKey` of an account
//! - View the `AccessKeyList` of an account
//! - Call a function in a contract deployed on the network.
//!
//! ## Examples
//!
//! ### Returns basic account information.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//! let request = methods::query::RpcQueryRequest {
//!     block_reference: BlockReference::BlockId(BlockId::Hash("6Qq9hYG7vQhnje4iC1hfbyhh9vNQoNem7j8Dxi7EVSdN".parse()?)),
//!     request: QueryRequest::ViewAccount {
//!         account_id: "itranscend.near".parse()?,
//!     }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::query::RpcQueryResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
//!
//! ### Returns the contract code (Wasm binary) deployed to the account. The returned code will be encoded in base64.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let request = methods::query::RpcQueryRequest {
//!     block_reference: BlockReference::BlockId(BlockId::Hash("CrYzVUyam5TMJTcJDJMSJ7Fzc79SDTgtK1SfVpEnteZF".parse()?)),
//!     request: QueryRequest::ViewCode {
//!         account_id: "nosedive.testnet".parse()?,
//!     }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::query::RpcQueryResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
//!
//! ### Returns the account state
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId, StoreKey}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let request = methods::query::RpcQueryRequest {
//!     // block_reference: BlockReference::BlockId(BlockId::Hash("AUDcb2iNUbsmCsmYGfGuKzyXKimiNcCZjBKTVsbZGnoH".parse()?)),
//!     block_reference: BlockReference::latest(),
//!     request: QueryRequest::ViewState {
//!         account_id: "nosedive.testnet".parse()?,
//!         prefix: StoreKey::from(vec![])
//!     }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::query::RpcQueryResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
//!
//! ### Returns information about a single access key for given account
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let request = methods::query::RpcQueryRequest {
//!     // block_reference: BlockReference::BlockId(BlockId::Hash("CA9bigchLBUYKaHKz3vQxK3Z7Fae2gnVabGrrLJrQEzp".parse()?)),
//!     block_reference: BlockReference::latest(),
//!     request: QueryRequest::ViewAccessKey {
//!         account_id: "fido.testnet".parse()?,
//!         public_key: "ed25519:GwRkfEckaADh5tVxe3oMfHBJZfHAJ55TRWqJv9hSpR38".parse()?
//!     }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::query::RpcQueryResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
//!
//! ### Returns all access keys for a given account.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let request = methods::query::RpcQueryRequest {
//!     block_reference: BlockReference::BlockId(BlockId::Hash("AUDcb2iNUbsmCsmYGfGuKzyXKimiNcCZjBKTVsbZGnoH".parse()?)),
//!     request: QueryRequest::ViewAccessKeyList {
//!         account_id: "nosedive.testnet".parse()?,
//!     }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::query::RpcQueryResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
//!
//! ### Call a function in a contract deployed on the network
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId, FunctionArgs}, views::QueryRequest};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let request = methods::query::RpcQueryRequest {
//!     // block_reference: BlockReference::BlockId(BlockId::Hash("CA9bigchLBUYKaHKz3vQxK3Z7Fae2gnVabGrrLJrQEzp".parse()?)),
//!     block_reference: BlockReference::latest(),
//!     request: QueryRequest::CallFunction {
//!         account_id: "nosedive.testnet".parse()?,
//!         method_name: "status".parse()?,
//!         args: FunctionArgs::from(
//!             json!({
//!                 "account_id": "miraclx.testnet",
//!             })
//!             .to_string()
//!             .into_bytes(),
//!         )
//!     }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::query::RpcQueryResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::query::{RpcQueryError, RpcQueryRequest, RpcQueryResponse};

impl RpcHandlerResponse for RpcQueryResponse {}

impl RpcHandlerError for RpcQueryError {}

impl private::Sealed for RpcQueryRequest {}

impl RpcMethod for RpcQueryRequest {
    type Response = RpcQueryResponse;
    type Error = RpcQueryError;

    fn method_name(&self) -> &str {
        "query"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }

    fn parse_handler_response(
        response: serde_json::Value,
    ) -> Result<Result<Self::Response, Self::Error>, serde_json::Error> {
        match serde_json::from_value::<QueryResponse>(response)? {
            QueryResponse::HandlerResponse(r) => Ok(Ok(r)),
            QueryResponse::HandlerError(LegacyQueryError {
                error,
                block_height,
                block_hash,
            }) => {
                let mut err_parts = error.split(' ');
                let query_error = if let (
                    Some("access"),
                    Some("key"),
                    Some(pk),
                    Some("does"),
                    Some("not"),
                    Some("exist"),
                    Some("while"),
                    Some("viewing"),
                    None,
                ) = (
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                ) {
                    let public_key = pk
                        .parse::<near_crypto::PublicKey>()
                        .map_err(serde::de::Error::custom)?;
                    RpcQueryError::UnknownAccessKey {
                        public_key,
                        block_height,
                        block_hash,
                    }
                } else {
                    RpcQueryError::ContractExecutionError {
                        vm_error: error,
                        block_height,
                        block_hash,
                    }
                };

                Ok(Err(query_error))
            }
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum QueryResponse {
    HandlerResponse(RpcQueryResponse),
    HandlerError(LegacyQueryError),
}

#[derive(serde::Deserialize)]
struct LegacyQueryError {
    error: String,
    block_height: near_primitives::types::BlockHeight,
    block_hash: near_primitives::hash::CryptoHash,
}

#[cfg(test)]
mod tests {
    use {super::*, crate::*};

    /// This test is to make sure the method executor treats `&RpcMethod`s the same as `RpcMethod`s.
    #[tokio::test]
    async fn test_unknown_method() -> Result<(), Box<dyn std::error::Error>> {
        let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

        let request = RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: near_primitives::views::QueryRequest::CallFunction {
                account_id: "testnet".parse()?,
                method_name: "some_unavailable_method".to_string(),
                args: vec![].into(),
            },
        };

        let response_err = client.call(&request).await.unwrap_err();

        assert!(
            matches!(
                response_err.handler_error(),
                Some(RpcQueryError::ContractExecutionError {
                    ref vm_error,
                    ..
                }) if vm_error.contains("MethodResolveError(MethodNotFound)")
            ),
            "this is unexpected: {:#?}",
            response_err
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_unknown_access_key() -> Result<(), Box<dyn std::error::Error>> {
        let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");

        let request = RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::BlockId(
                near_primitives::types::BlockId::Height(63503911),
            ),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: "miraclx.testnet".parse()?,
                public_key: "ed25519:9KnjTjL6vVoM8heHvCcTgLZ67FwFkiLsNtknFAVsVvYY".parse()?,
            },
        };

        let response_err = client.call(request).await.unwrap_err();

        assert!(
            matches!(
                response_err.handler_error(),
                Some(RpcQueryError::UnknownAccessKey {
                    ref public_key,
                    block_height: 63503911,
                    ..
                }) if public_key.to_string() == "ed25519:9KnjTjL6vVoM8heHvCcTgLZ67FwFkiLsNtknFAVsVvYY"
            ),
            "this is unexpected: {:#?}",
            response_err
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_contract_execution_error() -> Result<(), Box<dyn std::error::Error>> {
        let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");

        let request = RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::BlockId(
                near_primitives::types::BlockId::Height(63503911),
            ),
            request: near_primitives::views::QueryRequest::CallFunction {
                account_id: "miraclx.testnet".parse()?,
                method_name: "".to_string(),
                args: vec![].into(),
            },
        };

        let response_err = client.call(request).await.unwrap_err();

        assert!(
            matches!(
                response_err.handler_error(),
                Some(RpcQueryError::ContractExecutionError {
                    ref vm_error,
                    block_height: 63503911,
                    ..
                }) if vm_error.contains("MethodResolveError(MethodEmptyName)")
            ),
            "this is unexpected: {:#?}",
            response_err
        );

        Ok(())
    }
}
