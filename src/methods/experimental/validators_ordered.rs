//! Returns the ordered validators of a block.
//!
//! ## Example
//!
//! Returns the ordered validators for this [block](https://explorer.near.org/blocks/3Lq3Mtfpc3spH9oF5dXnUzvCBEqjTQwX1yCqKibwzgWR).
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::types::BlockId;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//! let request = methods::EXPERIMENTAL_validators_ordered::RpcValidatorsOrderedRequest {
//!     block_id: Some(BlockId::Hash("82x4wSUo9xjBJ8Lnk3iJ9Mcv3kULZkS3BuYijoYCEWUD".parse()?))
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_validators_ordered::RpcValidatorsOrderedResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::validator::{
    RpcValidatorError, RpcValidatorsOrderedRequest, RpcValidatorsOrderedResponse,
};

impl RpcHandlerResponse for RpcValidatorsOrderedResponse {}

impl RpcMethod for RpcValidatorsOrderedRequest {
    type Response = RpcValidatorsOrderedResponse;
    type Error = RpcValidatorError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_validators_ordered"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcValidatorsOrderedRequest {}
