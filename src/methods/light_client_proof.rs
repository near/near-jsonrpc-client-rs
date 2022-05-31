//! Returns the proofs for a transaction execution.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::types::TransactionOrReceiptId;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//! let request = methods::light_client_proof::RpcLightClientExecutionProofRequest {
//!     id: TransactionOrReceiptId::Transaction {
//!         transaction_hash: "47sXP4jKXCMpkUS6kcxsfNU7tqysYr5fxWFdEXQkZh6z".parse()?,
//!         sender_id: "aurora.pool.near".parse()?,
//!     },
//!     light_client_head: "ANm3jm5wq1Z4rJv6tXWyiDtC3wYKpXVHY4iq6bE1te7B".parse()?,
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::light_client_proof::RpcLightClientExecutionProofResponse { .. }
//! ));
//! Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::light_client::{
    RpcLightClientExecutionProofRequest, RpcLightClientExecutionProofResponse,
    RpcLightClientProofError,
};

impl RpcHandlerResponse for RpcLightClientExecutionProofResponse {}

impl RpcHandlerError for RpcLightClientProofError {
    fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcLightClientExecutionProofRequest {
    type Response = RpcLightClientExecutionProofResponse;
    type Error = RpcLightClientProofError;

    fn method_name(&self) -> &str {
        "light_client_proof"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcLightClientExecutionProofRequest {}
