//! Queries the status of a given node
//!
//! Returns the general status of a given node (sync status, nearcore node version, protocol version, etc), and the current set of validators.
//!
//! ## Example
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//! let request = methods::status::RpcStatusRequest;
//!
//! let response = client.call(request).await;
//!
//! assert!(
//!     matches!(response, Ok(methods::status::RpcStatusResponse { .. })),
//!     "expected an Ok(RpcStatusResponse) but got [{:?}]",
//!     response
//! );
//!
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::status::RpcStatusError;

pub type RpcStatusResponse = near_primitives::views::StatusResponse;

#[derive(Debug)]
pub struct RpcStatusRequest;

impl RpcHandlerResponse for RpcStatusResponse {}

impl RpcMethod for RpcStatusRequest {
    type Response = RpcStatusResponse;
    type Error = RpcStatusError;

    fn method_name(&self) -> &str {
        "status"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcStatusRequest {}
