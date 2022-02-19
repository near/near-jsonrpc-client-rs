use super::*;

pub use near_jsonrpc_primitives::types::sandbox::{
    RpcSandboxFastForwardError, RpcSandboxFastForwardRequest, RpcSandboxFastForwardResponse,
};

impl RpcHandlerResponse for RpcSandboxFastForwardResponse {}

impl RpcHandlerError for RpcSandboxFastForwardError {}

impl RpcMethod for RpcSandboxFastForwardRequest {
    type Response = RpcSandboxFastForwardResponse;
    type Error = RpcSandboxFastForwardError;

    fn method_name(&self) -> &str {
        "sandbox_fast_forward"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcSandboxFastForwardRequest {}
