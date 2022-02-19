use super::*;

pub use near_jsonrpc_primitives::types::sandbox::{
    RpcSandboxPatchStateError, RpcSandboxPatchStateRequest, RpcSandboxPatchStateResponse,
};

impl RpcHandlerResponse for RpcSandboxPatchStateResponse {}

impl RpcHandlerError for RpcSandboxPatchStateError {}

impl RpcMethod for RpcSandboxPatchStateRequest {
    type Response = RpcSandboxPatchStateResponse;
    type Error = RpcSandboxPatchStateError;

    fn method_name(&self) -> &str {
        "sandbox_patch_state"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcSandboxPatchStateRequest {}
