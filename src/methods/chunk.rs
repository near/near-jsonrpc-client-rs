//! Queries a chunk on the network.
//!
//! ## Examples
//!
//! Chunks can be queried using one of two ChunkReference variants: BlockShardId and ChunkHash.
//!
//! 1. BlockShardId: Get a chunk by it's BlockShardId. The BlockShardId is a combination of the block hash and the shard id.
//!    The block id field can either take in a block hash or a block height.
//!    
//!     a) Block hash:
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_jsonrpc_primitives::types::chunks;
//!     use near_primitives::types::BlockId;
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
//!
//!     let request = methods::chunk::RpcChunkRequest{
//!         chunk_reference: chunks::ChunkReference::BlockShardId {
//!             block_id: BlockId::Hash("6atGq4TUTZerVHU9qWoYfzXNBg3K4C4cca15TE6KfuBr".parse()?),
//!             shard_id: 0,
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::chunk::RpcChunkResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//!     b) Block height:
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_jsonrpc_primitives::types::chunks;
//!     use near_primitives::types::BlockId;
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
//!
//!     let request = methods::chunk::RpcChunkRequest{
//!         chunk_reference: chunks::ChunkReference::BlockShardId {
//!             block_id: BlockId::Height(61512623),
//!             shard_id: 3,
//!         }
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::chunk::RpcChunkResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//!
//! 2. ChunkHash: Get a chunk by it's ChunkHash.
//!    ```
//!    use near_jsonrpc_client::{methods, JsonRpcClient};
//!    use near_jsonrpc_primitives::types::chunks;
//!  
//!    # #[tokio::main]
//!    # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
//!  
//!    let request = methods::chunk::RpcChunkRequest{
//!        chunk_reference: chunks::ChunkReference::ChunkHash {
//!            chunk_id: "6GTgCQ5genLEEiPspEvdZEJooBzgWRrUnur9eGSdeTTD".parse()?,
//!        }
//!    };
//!  
//!    let response = client.call(request).await?;
//!  
//!    assert!(matches!(
//!        response,
//!        methods::chunk::RpcChunkResponse { .. }
//!    ));
//!    # Ok(())
//!    # }
//!    ```
use super::*;

pub use near_jsonrpc_primitives::types::chunks::{RpcChunkError, RpcChunkRequest};

pub type RpcChunkResponse = near_primitives::views::ChunkView;

impl RpcHandlerResponse for RpcChunkResponse {}

impl RpcHandlerError for RpcChunkError {
    fn parse_raw_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcChunkRequest {
    type Response = RpcChunkResponse;
    type Error = RpcChunkError;

    fn method_name(&self) -> &str {
        "chunk"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcChunkRequest {}
