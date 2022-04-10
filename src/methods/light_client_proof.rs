//! Verifies whether a transaction happened on chain.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::types::TransactionOrReceiptId;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("");
//!
//! let request = methods::light_client_proof::RpcLightClientExecutionProofRequest {
//!     id: TransactionOrReceiptId::Transaction {
//!             transaction_hash: "".parse()?,
//!             sender_id: "".parse()?,
//!         },
//!     light_client_head: "".parse()?,
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
    fn parse_raw_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
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
