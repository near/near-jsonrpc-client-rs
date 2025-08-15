//! Returns the gas price for a specific block height or block hash.
//!
//! ## Examples
//!
//! Returns the gas fees for this block:
//! <https://explorer.near.org/blocks/6atGq4TUTZerVHU9qWoYfzXNBg3K4C4cca15TE6KfuBr>
//!
//! - `BlockId::Height`
//!
//!     ```
//!     # use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::BlockId;
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//!
//!     let request = methods::gas_price::RpcGasPriceRequest {
//!         block_id: Some(BlockId::Height(61512623)),
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::gas_price::RpcGasPriceResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! - `BlockId::Hash`
//!
//!     ```
//!     # use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::BlockId;
//!
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//!
//!     let request = methods::gas_price::RpcGasPriceRequest {
//!         block_id: Some(BlockId::Hash("6atGq4TUTZerVHU9qWoYfzXNBg3K4C4cca15TE6KfuBr".parse()?)),
//!     };
//!
//!     let response = client.call(request).await?;
//!
//!     assert!(matches!(
//!         response,
//!         methods::gas_price::RpcGasPriceResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
use super::*;

pub use near_jsonrpc_primitives::types::gas_price::{RpcGasPriceError, RpcGasPriceRequest};

pub type RpcGasPriceResponse = near_primitives::views::GasPriceView;

impl RpcHandlerResponse for RpcGasPriceResponse {}

impl RpcHandlerError for RpcGasPriceError {
    fn parse(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        common::parse_unknown_block!(value => Self)
    }
}

impl RpcMethod for RpcGasPriceRequest {
    type Response = RpcGasPriceResponse;
    type Error = RpcGasPriceError;

    fn method_name(&self) -> &str {
        "gas_price"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!([self.block_id]))
    }
}

impl private::Sealed for RpcGasPriceRequest {}
