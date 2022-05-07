use super::*;

pub use near_jsonrpc_primitives::types::light_client::{
    RpcLightClientNextBlockError, RpcLightClientNextBlockRequest,
};
pub use near_primitives::views::LightClientBlockView;

pub type RpcLightClientNextBlockResponse = Option<LightClientBlockView>;

impl RpcHandlerResponse for RpcLightClientNextBlockResponse {}

impl RpcHandlerError for RpcLightClientNextBlockError {
    fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcLightClientNextBlockRequest {
    type Response = RpcLightClientNextBlockResponse;
    type Error = RpcLightClientNextBlockError;

    fn method_name(&self) -> &str {
        "next_light_client_block"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcLightClientNextBlockRequest {}
