use super::*;

pub use near_jsonrpc_primitives::types::config::{
    RpcProtocolConfigError, RpcProtocolConfigRequest,
};

pub type RpcProtocolConfigResponse = near_chain_configs::ProtocolConfigView;

impl RpcHandlerResponse for RpcProtocolConfigResponse {}

impl RpcHandlerError for RpcProtocolConfigError {
    fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcProtocolConfigRequest {
    type Response = RpcProtocolConfigResponse;
    type Error = RpcProtocolConfigError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_protocol_config"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }
}

impl private::Sealed for RpcProtocolConfigRequest {}
