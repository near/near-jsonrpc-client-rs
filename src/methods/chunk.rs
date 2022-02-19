use super::*;

pub use near_jsonrpc_primitives::types::chunks::{RpcChunkError, RpcChunkRequest};

pub type RpcChunkResponse = near_primitives::views::ChunkView;

impl RpcHandlerResponse for RpcChunkResponse {}

impl RpcHandlerError for RpcChunkError {
    fn parse_raw_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcChunkRequest {
    type Response = RpcChunkResponse;
    type Error = RpcChunkError;

    fn method_name(&self) -> &str {
        "chunk"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcChunkRequest {}
