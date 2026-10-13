//! Queries the status of a transaction, including the receipts it produced.
//!
//! Deprecated: nearcore 2.14 stabilized this method as `tx_status`; use
//! [`tx_status`](super::tx_status) instead. This variant keeps sending
//! `EXPERIMENTAL_tx_status`, which nodes of every version still serve, so it remains
//! usable while a network is still running nearcore 2.13.
//!
//! ## Example
//!
//! Returns the final transaction result for
//! <https://explorer.near.org/transactions/B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq>
//!
//! ```no_run
//! # #![allow(deprecated)]
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::views::TxExecutionStatus;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//! let tx_hash = "B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq".parse()?;
//!
//! let request = methods::EXPERIMENTAL_tx_status::RpcTransactionStatusRequest {
//!     transaction_info: methods::EXPERIMENTAL_tx_status::TransactionInfo::TransactionId {
//!         tx_hash,
//!         sender_account_id: "itranscend.near".parse()?,
//!     },
//!     wait_until: TxExecutionStatus::Executed,
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     near_jsonrpc_primitives::types::transactions::RpcTransactionResponse { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use near_jsonrpc_primitives::types::transactions::RpcTransactionResponse;
pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;

#[deprecated(
    since = "0.23.0",
    note = "nearcore 2.14 stabilized this method as `tx_status`; use `methods::tx_status::RpcTransactionStatusRequest`"
)]
#[derive(Debug)]
pub struct RpcTransactionStatusRequest {
    pub transaction_info: TransactionInfo,
    pub wait_until: near_primitives::views::TxExecutionStatus,
}

#[allow(deprecated)]
impl From<RpcTransactionStatusRequest>
    for near_jsonrpc_primitives::types::transactions::RpcTransactionStatusRequest
{
    fn from(this: RpcTransactionStatusRequest) -> Self {
        Self {
            transaction_info: this.transaction_info,
            wait_until: this.wait_until,
        }
    }
}

#[allow(deprecated)]
impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_tx_status"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(match &self.transaction_info {
            TransactionInfo::Transaction { signed_tx } => {
                json!({
                    "signed_tx_base64": common::serialize_signed_transaction(signed_tx)?,
                    "wait_until": self.wait_until
                })
            }
            TransactionInfo::TransactionId {
                tx_hash,
                sender_account_id,
            } => {
                json!({
                    "tx_hash": tx_hash,
                    "sender_account_id": sender_account_id,
                    "wait_until": self.wait_until
                })
            }
        })
    }
}

#[allow(deprecated)]
impl private::Sealed for RpcTransactionStatusRequest {}
