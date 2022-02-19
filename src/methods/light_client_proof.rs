use super::*;

pub use near_jsonrpc_primitives::types::light_client::{
    RpcLightClientExecutionProofRequest, RpcLightClientExecutionProofResponse,
    RpcLightClientProofError,
};

impl RpcHandlerResponse for RpcLightClientExecutionProofResponse {}

impl RpcHandlerError for RpcLightClientProofError {
    fn parse_raw_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcLightClientExecutionProofRequest {
    type Response = RpcLightClientExecutionProofResponse;
    type Error = RpcLightClientProofError;

    fn method_name(&self) -> &str {
        "light_client_proof"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcLightClientExecutionProofRequest {}
