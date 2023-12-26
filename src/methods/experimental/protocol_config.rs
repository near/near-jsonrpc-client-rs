//! Queries the protocol config of the blockchain at a given block.
//!
//! The `RpcProtocolConfigRequest` takes in a [`BlockReference`](https://docs.rs/near-primitives/0.12.0/near_primitives/types/enum.BlockReference.html) enum which has multiple variants.
//!
//! ## Example
//!
//! Returns the protocol config of the blockchain at a given block.
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::types::{BlockReference, BlockId};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//! let request = methods::EXPERIMENTAL_protocol_config::RpcProtocolConfigRequest {
//!     block_reference: BlockReference::BlockId(BlockId::Height(100_000_000))
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_protocol_config::RpcProtocolConfigResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::config::{
    RpcProtocolConfigError, RpcProtocolConfigRequest,
};

pub type RpcProtocolConfigResponse = near_chain_configs::ProtocolConfigView;

impl RpcHandlerResponse for RpcProtocolConfigResponse {}

impl RpcHandlerError for RpcProtocolConfigError {
    fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcProtocolConfigRequest {
    type Response = RpcProtocolConfigResponse;
    type Error = RpcProtocolConfigError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_protocol_config"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcProtocolConfigRequest {}
