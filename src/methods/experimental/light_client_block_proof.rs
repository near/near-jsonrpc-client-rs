//! Returns the proof that a block is part of the chain seen by a light client.
//!
//! ## Example
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//!
//! let request = methods::EXPERIMENTAL_light_client_block_proof::RpcLightClientBlockProofRequest {
//!     block_hash: "6Qq9hYG7vQhnje4iC1hfbyhh9vNQoNem7j8Dxi7EVSdN".parse()?,
//!     light_client_head: "ANm3jm5wq1Z4rJv6tXWyiDtC3wYKpXVHY4iq6bE1te7B".parse()?,
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_light_client_block_proof::RpcLightClientBlockProofResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::light_client::{
    RpcLightClientBlockProofRequest, RpcLightClientBlockProofResponse, RpcLightClientProofError,
};

impl RpcHandlerResponse for RpcLightClientBlockProofResponse {}

impl RpcMethod for RpcLightClientBlockProofRequest {
    type Response = RpcLightClientBlockProofResponse;
    type Error = RpcLightClientProofError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_light_client_block_proof"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcLightClientBlockProofRequest {}
