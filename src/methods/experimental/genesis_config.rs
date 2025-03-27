//! Queries the genesis config of the network.
//!
//! ## Example
//!
//! Returns the genesis config of the network.
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
//!
//! let request = methods::EXPERIMENTAL_genesis_config::RpcGenesisConfigRequest;
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::EXPERIMENTAL_genesis_config::RpcGenesisConfigResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub type RpcGenesisConfigResponse = near_chain_configs::GenesisConfig;

#[derive(Debug)]
pub struct RpcGenesisConfigRequest;

#[derive(Debug, Serialize, Deserialize, Error)]
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
