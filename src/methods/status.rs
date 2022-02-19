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
