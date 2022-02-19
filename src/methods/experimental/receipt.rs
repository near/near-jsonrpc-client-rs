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
