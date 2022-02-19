use super::*;

pub use near_jsonrpc_primitives::types::validator::{RpcValidatorError, RpcValidatorRequest};

pub type RpcValidatorResponse = near_primitives::views::EpochValidatorInfo;

impl RpcHandlerResponse for RpcValidatorResponse {}

impl RpcMethod for RpcValidatorRequest {
    type Response = RpcValidatorResponse;
    type Error = RpcValidatorError;

    fn method_name(&self) -> &str {
        "validators"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcValidatorRequest {}
