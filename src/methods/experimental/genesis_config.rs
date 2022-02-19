use super::*;

pub type RpcGenesisConfigResponse = near_chain_configs::GenesisConfig;

#[derive(Debug)]
pub struct RpcGenesisConfigRequest;

#[derive(Debug, Deserialize, Error)]
#[error("{}", unreachable!("fatal: this error should never be constructed"))]
pub enum RpcGenesisConfigError {}

impl RpcHandlerResponse for RpcGenesisConfigResponse {}

impl RpcHandlerError for RpcGenesisConfigError {}

impl RpcMethod for RpcGenesisConfigRequest {
    type Response = RpcGenesisConfigResponse;
    type Error = RpcGenesisConfigError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_genesis_config"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcGenesisConfigRequest {}
