use super::*;

pub use near_jsonrpc_primitives::types::changes::{
    RpcStateChangesError, RpcStateChangesInBlockByTypeRequest, RpcStateChangesInBlockResponse,
};

impl RpcHandlerResponse for RpcStateChangesInBlockResponse {}

impl RpcMethod for RpcStateChangesInBlockByTypeRequest {
    type Response = RpcStateChangesInBlockResponse;
    type Error = RpcStateChangesError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_changes"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcStateChangesInBlockByTypeRequest {}
