//! Queries data from a specific block on the network.
//!
//! ## Examples
//!
//! Blocks can be queried using either one of three variants: BlockID, Finality and SyncCheckpoint.
//!
//! 1. BlockId: The BlockId enum accepts a BlockHeight or a BlockHash as a variant.
//!
//!     a) BlockId::Height : Allows you to specify the height of the block you want to query.
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::{BlockReference, BlockId};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//!     let request = methods::block::RpcBlockRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Height(83975193))
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::block::RpcBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//!     b) BlockId::Hash : Allows you to specify the hash of the block you want to query.
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::{types::{BlockReference, BlockId}, hash::CryptoHash};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//!     let request = methods::block::RpcBlockRequest {
//!         block_reference: BlockReference::BlockId(BlockId::Hash("G1SHrwLp55oV3kz94x3ekrR6r4ihNRWdAVZpckgBx4U4".parse()?))
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::block::RpcBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! 2. Finality: The Finality enum accepts a Finality::None (for optimistic finality) or a Finality::Final (final finality) as a variant.
//!
//!     a) Optimistic finality (Finality::None) : Returns the latest block recorded on the node that responded to your query (<1 second delay after the transaction is submitted).
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::{BlockReference, Finality};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//!     let request = methods::block::RpcBlockRequest {
//!         block_reference: BlockReference::Finality(Finality::None)
//!     };
//!
//!     let response = client.call(request).await?;
//!     
//!     assert!(matches!(
//!         response,
//!         methods::block::RpcBlockResponse { .. }
//!     ));   
//!     # Ok(())
//!     # }
//!     ```
//!
//!     b) Final finality: (Finality::Final) : Returns the latest finalised block (ie. a block that has been validated on at least 66% of the nodes in the network [usually takes 2 blocks / approx. 2 second delay]).
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::{BlockReference, Finality};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//!     let request = methods::block::RpcBlockRequest {
//!         block_reference: BlockReference::Finality(Finality::Final)
//!     };
//!
//!     let response = client.call(request).await?;
//!     
//!      assert!(matches!(
//!         response,
//!         methods::block::RpcBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! 3. SyncCheckpoint: Queries blocks by their SyncCheckpoint. The checkpoint could either be the first checkpoint after the network's genesis block or the earliest availabe checkpoint.
//!
//!     a) SyncCheckpoint::Genesis : Returns the block at the first Sync Checkpoint after the network's genesis block.
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::{BlockReference, SyncCheckpoint};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
//!
//!     let request = methods::block::RpcBlockRequest {
//!         block_reference: BlockReference::SyncCheckpoint(SyncCheckpoint::Genesis)
//!     };
//!
//!     let response = client.call(request).await?;
//!     
//!      assert!(matches!(
//!         response,
//!         methods::block::RpcBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//!     b) SyncCheckpoint::EarliestAvailable : Returns the block at the network's most recent Sync Checkpoint.
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::{BlockReference, SyncCheckpoint};
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//!     let request = methods::block::RpcBlockRequest {
//!         block_reference: BlockReference::SyncCheckpoint(SyncCheckpoint::EarliestAvailable)
//!     };
//!
//!     let response = client.call(request).await?;
//!     
//!      assert!(matches!(
//!         response,
//!         methods::block::RpcBlockResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
use super::*;

pub use near_jsonrpc_primitives::types::blocks::RpcBlockError;
pub use near_jsonrpc_primitives::types::blocks::RpcBlockRequest;

pub type RpcBlockResponse = near_primitives::views::BlockView;

impl RpcHandlerResponse for RpcBlockResponse {}

impl RpcHandlerError for RpcBlockError {
    fn parse_raw_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcBlockRequest {
    type Response = RpcBlockResponse;
    type Error = RpcBlockError;

    fn method_name(&self) -> &str {
        "block"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcBlockRequest {}
