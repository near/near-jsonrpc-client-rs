//! Returns account changes from transactions in a given account.
//!
//! The `RpcStateChangesInBlockByTypeRequest` struct takes in a `BlockReference` and a `StateChangesRequestView`, and returns an `RpcStateChangesInBlockResponse`.
//!
//! ## Examples
//!
//! The `StateChangesRequestView` enum has a couple of variants that can be used to specify what kind of changes to return.
//!
//! - `AccountChanges`
//!
//!     ```
//!     # use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::{views::StateChangesRequestView, types::{BlockReference, BlockId}};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//!     let request = methods::EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Hash("94yBWhN848vHMnKcw5DxgBQWJW6JHRXnXD6FCLJGjxMU".parse()?)),
//!         state_changes_request: StateChangesRequestView::AccountChanges {
//!            account_ids: vec!["fido.testnet".parse()?, "rpc_docs.testnet".parse()?],
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::EXPERIMENTAL_changes::RpcStateChangesInBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! - `SingleAccessKeyChanges`
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::{views::StateChangesRequestView, types::{BlockReference, BlockId, AccountWithPublicKey}};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//!     let request = methods::EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Hash("94yBWhN848vHMnKcw5DxgBQWJW6JHRXnXD6FCLJGjxMU".parse()?)),
//!         state_changes_request: StateChangesRequestView::SingleAccessKeyChanges {
//!            keys: vec![
//!                     AccountWithPublicKey {
//!                         account_id: "fido.testnet".parse()?,
//!                         public_key: "ed25519:GwRkfEckaADh5tVxe3oMfHBJZfHAJ55TRWqJv9hSpR38".parse()?,
//!                     },
//!                     AccountWithPublicKey {
//!                         account_id: "rpc_docs.testnet".parse()?,
//!                         public_key: "ed25519:FxGiXr6Dgn92kqBqbQzuoYdKngiizCnywpaN7ALar3Vv".parse()?,
//!                     }
//!
//!                ],
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::EXPERIMENTAL_changes::RpcStateChangesInBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! - `AllAccessKeyChanges`
//!
//!     ```
//!     # use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::{views::StateChangesRequestView, types::{BlockReference, BlockId}};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//!     let request = methods::EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Hash("94yBWhN848vHMnKcw5DxgBQWJW6JHRXnXD6FCLJGjxMU".parse()?)),
//!         state_changes_request: StateChangesRequestView::AllAccessKeyChanges {
//!            account_ids: vec!["fido.testnet".parse()?, "rpc_docs.testnet".parse()?],
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::EXPERIMENTAL_changes::RpcStateChangesInBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! - `ContractCodeChanges`
//!
//!     ```
//!     # use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::{views::StateChangesRequestView, types::{BlockReference, BlockId}};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//!     let request = methods::EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Hash("94yBWhN848vHMnKcw5DxgBQWJW6JHRXnXD6FCLJGjxMU".parse()?)),
//!         state_changes_request: StateChangesRequestView::ContractCodeChanges {
//!            account_ids: vec!["fido.testnet".parse()?, "rpc_docs.testnet".parse()?],
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::EXPERIMENTAL_changes::RpcStateChangesInBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! - `DataChanges`
//!
//!     ```
//!     # use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::{views::StateChangesRequestView, types::{BlockReference, BlockId, StoreKey}, hash::CryptoHash};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//!     let request = methods::EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Hash("94yBWhN848vHMnKcw5DxgBQWJW6JHRXnXD6FCLJGjxMU".parse::<CryptoHash>()?)),
//!         state_changes_request: StateChangesRequestView::DataChanges {
//!            account_ids: vec!["fido.testnet".parse()?, "rpc_docs.testnet".parse()?],
//!            key_prefix: StoreKey::from(vec![]),
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::EXPERIMENTAL_changes::RpcStateChangesInBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
use super::*;

pub use near_jsonrpc_primitives::types::changes::{
    RpcStateChangesError, RpcStateChangesInBlockByTypeRequest, RpcStateChangesInBlockResponse,
};

impl RpcHandlerResponse for RpcStateChangesInBlockResponse {}

impl RpcMethod for RpcStateChangesInBlockByTypeRequest {
    type Response = RpcStateChangesInBlockResponse;
    type Error = RpcStateChangesError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_changes"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcStateChangesInBlockByTypeRequest {}
