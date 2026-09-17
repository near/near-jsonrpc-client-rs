//! Returns a proof of a state item (account, contract code, contract data or access
//! key) against the post-state root of a certified chunk (nearcore 2.14+).
//!
//! ## Example
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::types::{ShardId, SpiceChunkId};
//! use near_primitives::views::StateProofTarget;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//!
//! let request = methods::EXPERIMENTAL_light_client_state_proof::RpcLightClientStateProofRequest {
//!     chunk_id: SpiceChunkId {
//!         block_hash: "6Qq9hYG7vQhnje4iC1hfbyhh9vNQoNem7j8Dxi7EVSdN".parse()?,
//!         shard_id: ShardId::new(0),
//!     },
//!     target: StateProofTarget::Account {
//!         account_id: "aurora".parse()?,
//!     },
//!     light_client_head: "ANm3jm5wq1Z4rJv6tXWyiDtC3wYKpXVHY4iq6bE1te7B".parse()?,
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_light_client_state_proof::RpcLightClientStateProofResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::light_client::{
    RpcLightClientProofError, RpcLightClientStateProofRequest, RpcLightClientStateProofResponse,
};
pub use near_primitives::views::StateProofTarget;

impl RpcHandlerResponse for RpcLightClientStateProofResponse {}

impl RpcMethod for RpcLightClientStateProofRequest {
    type Response = RpcLightClientStateProofResponse;
    type Error = RpcLightClientProofError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_light_client_state_proof"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcLightClientStateProofRequest {}
