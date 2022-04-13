//! Patch account, access keys, contract code, or contract state.
//!
//! Only additions and mutations are supported. No deletions. 
//! 
//! Account, access keys, contract code, and contract states have different formats. See the example and docs for details about their format.
//!
//! ## Examples
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::{state_record::StateRecord, account, types::{AccountId, StorageUsage}};
//! use near_primitives::{account::AccountVersion, hash::CryptoHash};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("http://localhost:3030");
//!
//! let request = methods::sandbox_patch_state::RpcSandboxPatchStateRequest {
//!     records: vec![
//!         StateRecord::Account {
//!             account_id: "fido.testnet".parse::<AccountId>()?,
//!             account: account::Account::new(179, 0, CryptoHash::default(), 264)
//!          }
//!     ],
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     methods::sandbox_patch_state::RpcSandboxPatchStateResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
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
