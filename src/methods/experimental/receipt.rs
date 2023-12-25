//! Fetches a receipt by it's ID
//!
//! The `RpcReceiptRequest` takes in a [`ReceiptReference`](https://docs.rs/near-jsonrpc-primitives/0.12.0/near_jsonrpc_primitives/types/receipts/struct.ReceiptReference.html)
//!
//! ## Example
//!
//! Returns the receipt for this [transaction](https://explorer.near.org/transactions/4nVcmhWkV8Y3uJp9VQWrJhfesncJERfrvt9WwDi77oEJ#3B5PPT9EKj5352Wks9GnCeSUBDsVvSF4ceMQv2nEULTf) on mainnet.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_jsonrpc_primitives::types::receipts::ReceiptReference;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//! let request = methods::EXPERIMENTAL_receipt::RpcReceiptRequest {
//!     receipt_reference: ReceiptReference {
//!         receipt_id: "3B5PPT9EKj5352Wks9GnCeSUBDsVvSF4ceMQv2nEULTf".parse()?,
//!     }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_receipt::RpcReceiptResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::receipts::{RpcReceiptError, RpcReceiptRequest};

pub type RpcReceiptResponse = near_primitives::views::ReceiptView;

impl RpcHandlerResponse for RpcReceiptResponse {}

impl RpcHandlerError for RpcReceiptError {}

impl RpcMethod for RpcReceiptRequest {
    type Response = RpcReceiptResponse;
    type Error = RpcReceiptError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_receipt"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcReceiptRequest {}
