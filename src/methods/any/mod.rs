//! For all intents and purposes, the predefined structures in `methods` should suffice, if you find that they
//! don't or you crave extra flexibility, well, you can opt in to use the generic constructor `methods::any()` with the `any` feature flag.
//!
//! In this example, we retrieve only the parts from the genesis config response that we care about.
//!
//! ```toml
//! # in Cargo.toml
//! near-jsonrpc-client = { ..., features = ["any"] }
//! ```
//!
//! ```
//! use serde::Deserialize;
//! use serde_json::json;
//!
//! # use near_jsonrpc_client::errors::JsonRpcError;
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::serialize::u128_dec_format;
//! use near_primitives::types::*;
//!
//! #[derive(Debug, Deserialize)]
//! struct PartialGenesisConfig {
//!     protocol_version: ProtocolVersion,
//!     chain_id: String,
//!     genesis_height: BlockHeight,
//!     epoch_length: BlockHeightDelta,
//!     #[serde(with = "u128_dec_format")]
//!     min_gas_price: Balance,
//!     #[serde(with = "u128_dec_format")]
//!     max_gas_price: Balance,
//!     #[serde(with = "u128_dec_format")]
//!     total_supply: Balance,
//!     validators: Vec<AccountInfo>,
//! }
//!
//! impl methods::RpcHandlerResponse for PartialGenesisConfig {}
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), JsonRpcError<()>> {
//! let client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
//!
//! # #[cfg(feature = "any")] {
//! let genesis_config_request = methods::any::<Result<PartialGenesisConfig, ()>>(
//!     "EXPERIMENTAL_genesis_config",
//!     json!(null),
//! );
//!
//! let partial_genesis = client.call(genesis_config_request).await?;
//!
//! println!("{:#?}", partial_genesis);
//! # }
//! # Ok(())
//! # }
//! ```
use super::*;

use std::marker::PhantomData;

pub fn request<T: AnyRequestResult>(
    method_name: &str,
    params: serde_json::Value,
) -> RpcAnyRequest<T::Response, T::Error>
where
    T::Response: RpcHandlerResponse,
    T::Error: RpcHandlerError,
{
    RpcAnyRequest {
        method: method_name.to_string(),
        params,
        _data: PhantomData,
    }
}

#[derive(Debug)]
pub struct RpcAnyRequest<T, E> {
    pub method: String,
    pub params: serde_json::Value,
    pub(crate) _data: PhantomData<(T, E)>,
}

impl<T, E> private::Sealed for RpcAnyRequest<T, E> {}

impl<T, E> RpcMethod for RpcAnyRequest<T, E>
where
    T: RpcHandlerResponse,
    E: RpcHandlerError,
{
    type Response = T;
    type Error = E;

    #[inline(always)]
    fn method_name(&self) -> &str {
        &self.method
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(self.params.clone())
    }
}

pub trait AnyRequestResult {
    type Response;
    type Error;
}

impl<T, E> AnyRequestResult for Result<T, E> {
    type Response = T;
    type Error = E;
}

impl<T: RpcMethod> AnyRequestResult for T {
    type Response = T::Response;
    type Error = T::Error;
}
