//! Returns the next light client block.
//!
//! ## Example
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//! let request = methods::next_light_client_block::RpcLightClientNextBlockRequest {
//!     last_block_hash: "ANm3jm5wq1Z4rJv6tXWyiDtC3wYKpXVHY4iq6bE1te7B".parse()?,
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     Some(methods::next_light_client_block::LightClientBlockView { .. })
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::light_client::{
    RpcLightClientNextBlockError, RpcLightClientNextBlockRequest,
};
pub use near_primitives::views::LightClientBlockView;

pub type RpcLightClientNextBlockResponse = Option<LightClientBlockView>;

impl RpcHandlerResponse for RpcLightClientNextBlockResponse {}

impl RpcHandlerError for RpcLightClientNextBlockError {
    fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcLightClientNextBlockRequest {
    type Response = RpcLightClientNextBlockResponse;
    type Error = RpcLightClientNextBlockError;

    fn method_name(&self) -> &str {
        "next_light_client_block"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcLightClientNextBlockRequest {}
