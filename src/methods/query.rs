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
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
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
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");
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
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId, StoreKey}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");
//!
//! let request = methods::query::RpcQueryRequest {
//!     // block_reference: BlockReference::BlockId(BlockId::Hash("AUDcb2iNUbsmCsmYGfGuKzyXKimiNcCZjBKTVsbZGnoH".parse()?)),
//!     block_reference: BlockReference::latest(),
//!     request: QueryRequest::ViewState {
//!         account_id: "nosedive.testnet".parse()?,
//!         prefix: StoreKey::from(vec![]),
//!         after_key: None,
//!         limit: None,
//!         include_proof: false,
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
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");
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
//! ### Returns the access keys of a given account, one page at a time.
//!
//! Since nearcore 2.14 (protocol 87) `view_access_key_list` is paginated: a response
//! holds at most `limit` keys (capped by the node), and `last_key` is the cursor to
//! pass as `after_key` to fetch the next page. `last_key` is `None` on the final page.
//! A request with neither `limit` nor `after_key` is the legacy unpaginated form and
//! returns [`RpcQueryError::TooManyAccessKeys`] once the account holds more keys than
//! the node's cap, so set `limit` from the first request on.
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_jsonrpc_primitives::types::query::QueryResponseKind;
//! use near_primitives::{types::{BlockReference, BlockId}, views::QueryRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");
//!
//! let mut after_key = None;
//! loop {
//!     let request = methods::query::RpcQueryRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Hash("AUDcb2iNUbsmCsmYGfGuKzyXKimiNcCZjBKTVsbZGnoH".parse()?)),
//!         request: QueryRequest::ViewAccessKeyList {
//!             account_id: "nosedive.testnet".parse()?,
//!             after_key,
//!             limit: Some(50.try_into()?),
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     let QueryResponseKind::AccessKeyList(page) = response.kind else {
//!         panic!("unexpected query response kind");
//!     };
//!     for key in page.keys {
//!         println!("{}", key.public_key);
//!     }
//!     match page.last_key {
//!         Some(last_key) => after_key = Some(last_key),
//!         None => break,
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Call a function in a contract deployed on the network
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{types::{BlockReference, BlockId, FunctionArgs}, views::QueryRequest};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");
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
                        vm_error: error.clone(),
                        error: near_primitives::errors::FunctionCallError::ExecutionError(error),
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
                    vm_error,
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
        let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");

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
                    public_key,
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
        let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");

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
                    vm_error,
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

#[cfg(test)]
mod pagination_tests {
    use super::*;

    #[test]
    fn view_access_key_list_omits_unset_pagination_fields() {
        let request = RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKeyList {
                account_id: "nosedive.testnet".parse().unwrap(),
                after_key: None,
                limit: None,
            },
        };
        let params = request.params().unwrap();
        assert_eq!(params["request_type"], "view_access_key_list");
        assert!(params.get("after_key").is_none());
        assert!(params.get("limit").is_none());
    }

    #[test]
    fn view_access_key_list_serializes_pagination_fields() {
        let after_key = "ed25519:GwRkfEckaADh5tVxe3oMfHBJZfHAJ55TRWqJv9hSpR38"
            .parse::<near_crypto::PublicKeyHandle>()
            .unwrap();
        let request = RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKeyList {
                account_id: "nosedive.testnet".parse().unwrap(),
                after_key: Some(after_key),
                limit: Some(25.try_into().unwrap()),
            },
        };
        let params = request.params().unwrap();
        assert_eq!(
            params["after_key"],
            "ed25519:GwRkfEckaADh5tVxe3oMfHBJZfHAJ55TRWqJv9hSpR38"
        );
        assert_eq!(params["limit"], 25);
    }

    #[test]
    fn access_key_list_response_carries_last_key() {
        let response: RpcQueryResponse = serde_json::from_value(serde_json::json!({
            "block_height": 1,
            "block_hash": "11111111111111111111111111111111",
            "keys": [],
            "last_key": "ed25519:GwRkfEckaADh5tVxe3oMfHBJZfHAJ55TRWqJv9hSpR38",
        }))
        .unwrap();
        let near_jsonrpc_primitives::types::query::QueryResponseKind::AccessKeyList(list) =
            response.kind
        else {
            panic!("expected AccessKeyList");
        };
        assert!(list.keys.is_empty());
        assert_eq!(
            list.last_key.unwrap().to_string(),
            "ed25519:GwRkfEckaADh5tVxe3oMfHBJZfHAJ55TRWqJv9hSpR38"
        );
    }
}
