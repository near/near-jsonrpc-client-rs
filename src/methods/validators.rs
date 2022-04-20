//! Queries active validators on the network.
//!
//! Returns details and the state of validation on the blockchain.
//!
//! ## Examples
//!
//! - Get the validators for a specified epoch.
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::{EpochReference, EpochId, BlockReference, Finality};
//!   
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!   
//!     let block_request = methods::block::RpcBlockRequest {
//!         block_reference: BlockReference::Finality(Finality::Final),
//!     };
//!     let block_response = client.call(block_request).await?;
//!     let epoch_hash = block_response.header.epoch_id;
//!   
//!     let request = methods::validators::RpcValidatorRequest {
//!         epoch_reference: EpochReference::EpochId(EpochId {
//!             0: epoch_hash,
//!         })
//!     };
//!   
//!     let response = client.call(request).await?;
//!   
//!     assert!(matches!(
//!         response,
//!         methods::validators::RpcValidatorResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
//!
//! - Get the validators for the latest block
//!
//!     ```
//!     use near_jsonrpc_client::{methods, JsonRpcClient};
//!     use near_primitives::types::{EpochReference, EpochId, BlockId};
//!   
//!     # #[tokio::main]
//!     # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!   
//!     let request = methods::validators::RpcValidatorRequest {
//!         epoch_reference: EpochReference::Latest
//!     };
//!   
//!     let response = client.call(request).await?;
//!   
//!     assert!(matches!(
//!         response,
//!         methods::validators::RpcValidatorResponse { .. }
//!     ));
//!     # Ok(())
//!     # }
//!     ```
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
