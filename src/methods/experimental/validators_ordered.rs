use super::*;

pub use near_jsonrpc_primitives::types::validator::{
    RpcValidatorError, RpcValidatorsOrderedRequest, RpcValidatorsOrderedResponse,
};

impl RpcHandlerResponse for RpcValidatorsOrderedResponse {}

impl RpcMethod for RpcValidatorsOrderedRequest {
    type Response = RpcValidatorsOrderedResponse;
    type Error = RpcValidatorError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_validators_ordered"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcValidatorsOrderedRequest {}
