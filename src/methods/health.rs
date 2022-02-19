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
