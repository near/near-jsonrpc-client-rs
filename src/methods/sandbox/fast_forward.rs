//! Fast forwards a sandboxed node by a specific height.
//!
//! Fas forwarding allows one to skip to some point in the future and observe actions.
//!
//! ## Example
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("http://localhost:3030");
//!
//! let request = methods::sandbox_fast_forward::RpcSandboxFastForwardRequest {
//!     delta_height: 12,
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::sandbox_fast_forward::RpcSandboxFastForwardResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::sandbox::{
    RpcSandboxFastForwardError, RpcSandboxFastForwardRequest, RpcSandboxFastForwardResponse,
};

impl RpcHandlerResponse for RpcSandboxFastForwardResponse {}

impl RpcHandlerError for RpcSandboxFastForwardError {}

impl RpcMethod for RpcSandboxFastForwardRequest {
    type Response = RpcSandboxFastForwardResponse;
    type Error = RpcSandboxFastForwardError;

    fn method_name(&self) -> &str {
        "sandbox_fast_forward"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcSandboxFastForwardRequest {}
