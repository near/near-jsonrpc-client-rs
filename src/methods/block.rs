//! Queries data from a specific block on the network.
//!
//! Blocks can be referenced using either;
//! - a [block ID](https://docs.near.org/docs/api/rpc#using-block_id-param) (block height or block hash) for querying historical blocks
//! - or a [finality specifier](https://docs.near.org/docs/api/rpc#using-finality-param) (“final” or “optimistic”) for latest blocks.
//!
//! ## Examples
//!
//! - Query historical blocks by using a specific reference (block height or block hash).
//!
//!     - `BlockId::Height`
//!
//!       ```
//!       # use near_jsonrpc_client::methods;
//!       use near_primitives::types::{BlockReference, BlockId};
//!
//!       let request = methods::block::RpcBlockRequest {
//!           block_reference: BlockReference::BlockId(BlockId::Height(83975193))
//!       };
//!       ```
//!
//!     - `BlockId::Hash`
//!
//!       ```
//!       # use near_jsonrpc_client::methods;
//!       use near_primitives::types::{BlockReference, BlockId};
//!
//!       let request = methods::block::RpcBlockRequest {
//!           block_reference: BlockReference::BlockId(BlockId::Hash(
//!               "G1SHrwLp55oV3kz94x3ekrR6r4ihNRWdAVZpckgBx4U4".parse()?,
//!           )),
//!       };
//!       ```
//!
//! - Query latest blocks.
//!
//!     - `Finality::Final`: Get the most recent, completely finalized block.
//!
//!       References a block that has been validated on at least 66% of the nodes in the network.
//!
//!       ```
//!       # use near_jsonrpc_client::methods;
//!       use near_primitives::types::{BlockReference, Finality};
//!
//!       let request = methods::block::RpcBlockRequest {
//!           block_reference: BlockReference::Finality(Finality::Final)
//!       };
//!       ```
//!
//!     - `Finality::None`: Get the most recently submitted block.
//!
//!       Returns the latest block recorded on the node that responded to your query.
//!
//!       ```
//!       # use near_jsonrpc_client::methods;
//!       use near_primitives::types::{BlockReference, Finality};
//!
//!       let request = methods::block::RpcBlockRequest {
//!           block_reference: BlockReference::Finality(Finality::None)
//!       };
//!       ```
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
