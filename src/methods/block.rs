use super::*;

pub use near_jsonrpc_primitives::types::blocks::RpcBlockError;
pub use near_jsonrpc_primitives::types::blocks::RpcBlockRequest;

pub type RpcBlockResponse = near_primitives::views::BlockView;

impl RpcHandlerResponse for RpcBlockResponse {}

impl RpcHandlerError for RpcBlockError {
    fn parse_raw_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcBlockRequest {
    type Response = RpcBlockResponse;
    type Error = RpcBlockError;

    fn method_name(&self) -> &str {
        "block"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcBlockRequest {}
