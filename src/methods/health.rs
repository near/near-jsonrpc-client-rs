//! Requests health status of the rpc node.
//!
//! ## Example
//!
//! Returns the current health stauts of the rpc node it connects to.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//! let request = methods::health::RpcHealthRequest;
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::health::RpcHealthResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::status::{RpcHealthResponse, RpcStatusError};

#[derive(Debug)]
pub struct RpcHealthRequest;

impl RpcHandlerResponse for RpcHealthResponse {}

impl RpcMethod for RpcHealthRequest {
    type Response = RpcHealthResponse;
    type Error = RpcStatusError;

    fn method_name(&self) -> &str {
        "health"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcHealthRequest {}
