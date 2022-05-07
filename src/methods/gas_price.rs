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
