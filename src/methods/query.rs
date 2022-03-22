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

impl RpcMethod for RpcQueryRequest {
    type Response = RpcQueryResponse;
    type Error = RpcQueryError;

    fn method_name(&self) -> &str {
        "query"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcQueryRequest {}
