//! Returns the changes in a block.
//!
//! The `RpcStateChangesInBlockRequest` takes in a [`BlockReference`](https://docs.rs/near-primitives/0.12.0/near_primitives/types/enum.BlockReference.html) enum which has multiple variants.
//!
//! ## Example
//!
//! Returns the changes in block for <https://explorer.near.org/blocks/3Lq3Mtfpc3spH9oF5dXnUzvCBEqjTQwX1yCqKibwzgWR>
//!
//! You can also use the `Finality` and `SyncCheckpoint` variants of [`BlockReference`](https://docs.rs/near-primitives/0.12.0/near_primitives/types/enum.BlockReference.html) to return block change details.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::types::{BlockReference, BlockId};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//!
//! let request = methods::EXPERIMENTAL_changes_in_block::RpcStateChangesInBlockRequest {
//!     block_reference: BlockReference::BlockId(BlockId::Height(47988413))
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_changes_in_block::RpcStateChangesInBlockByTypeResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::changes::{
    RpcStateChangesError, RpcStateChangesInBlockByTypeResponse, RpcStateChangesInBlockRequest,
};

impl RpcHandlerResponse for RpcStateChangesInBlockByTypeResponse {}

impl RpcMethod for RpcStateChangesInBlockRequest {
    type Response = RpcStateChangesInBlockByTypeResponse;
    type Error = RpcStateChangesError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_changes_in_block"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcStateChangesInBlockRequest {}
