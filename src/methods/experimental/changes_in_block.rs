use super::*;

pub use near_jsonrpc_primitives::types::changes::{
    RpcStateChangesError, RpcStateChangesInBlockByTypeResponse, RpcStateChangesInBlockRequest,
};

impl RpcHandlerResponse for RpcStateChangesInBlockByTypeResponse {}

impl RpcMethod for RpcStateChangesInBlockRequest {
    type Response = RpcStateChangesInBlockByTypeResponse;
    type Error = RpcStateChangesError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_changes_in_block"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcStateChangesInBlockRequest {}
