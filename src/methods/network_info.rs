use super::*;

pub use near_jsonrpc_primitives::types::network_info::{
    RpcNetworkInfoError, RpcNetworkInfoResponse,
};

#[derive(Debug)]
pub struct RpcNetworkInfoRequest;

impl RpcHandlerResponse for RpcNetworkInfoResponse {}

impl RpcHandlerError for RpcNetworkInfoError {}

impl RpcMethod for RpcNetworkInfoRequest {
    type Response = RpcNetworkInfoResponse;
    type Error = RpcNetworkInfoError;

    fn method_name(&self) -> &str {
        "network_info"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcNetworkInfoRequest {}
