use super::*;

pub use near_jsonrpc_primitives::types::query::{RpcQueryError, RpcQueryRequest, RpcQueryResponse};

impl RpcHandlerResponse for RpcQueryResponse {}

impl RpcHandlerError for RpcQueryError {}

impl RpcMethod for RpcQueryRequest {
    type Response = RpcQueryResponse;
    type Error = RpcQueryError;

    fn method_name(&self) -> &str {
        "query"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcQueryRequest {}
