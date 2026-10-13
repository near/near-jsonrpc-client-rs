//! Returns the proof of a transaction or receipt outcome, anchored in the chunk
//! execution proof of the chunk that produced it (nearcore 2.14+).
//!
//! ## Example
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::types::TransactionOrReceiptId;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//!
//! let request =
//!     methods::EXPERIMENTAL_light_client_execution_outcome_proof::RpcLightClientExecutionOutcomeProofRequest {
//!         id: TransactionOrReceiptId::Transaction {
//!             transaction_hash: "47sXP4jKXCMpkUS6kcxsfNU7tqysYr5fxWFdEXQkZh6z".parse()?,
//!             sender_id: "aurora.pool.near".parse()?,
//!         },
//!         light_client_head: "ANm3jm5wq1Z4rJv6tXWyiDtC3wYKpXVHY4iq6bE1te7B".parse()?,
//!     };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_light_client_execution_outcome_proof::RpcLightClientExecutionOutcomeProofResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::light_client::{
    RpcLightClientExecutionOutcomeProofRequest, RpcLightClientExecutionOutcomeProofResponse,
    RpcLightClientProofError,
};

impl RpcHandlerResponse for RpcLightClientExecutionOutcomeProofResponse {}

impl RpcMethod for RpcLightClientExecutionOutcomeProofRequest {
    type Response = RpcLightClientExecutionOutcomeProofResponse;
    type Error = RpcLightClientProofError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_light_client_execution_outcome_proof"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcLightClientExecutionOutcomeProofRequest {}
