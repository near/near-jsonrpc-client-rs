//! Queries the status of a transaction, including the receipts it produced.
//!
//! This is the stable form of the `EXPERIMENTAL_tx_status` method (nearcore 2.14 /
//! protocol 87 stabilized it as `tx_status`). Unlike [`tx`](super::tx), the response
//! carries the receipts alongside the outcome
//! ([`FinalExecutionOutcomeViewEnum::FinalExecutionOutcomeWithReceipt`](near_primitives::views::FinalExecutionOutcomeViewEnum::FinalExecutionOutcomeWithReceipt)).
//!
//! Nodes older than 2.14 do not serve `tx_status`; against those, use
//! [`EXPERIMENTAL_tx_status`](super::EXPERIMENTAL_tx_status), which every node still accepts.
//!
//! ## Example
//!
//! Returns the final transaction result for
//! <https://explorer.near.org/transactions/B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq>
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::views::TxExecutionStatus;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.fastnear.com");
//! let tx_hash = "B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq".parse()?;
//!
//! let request = methods::tx_status::RpcTransactionStatusRequest {
//!     transaction_info: methods::tx_status::TransactionInfo::TransactionId {
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
pub use near_jsonrpc_primitives::types::transactions::TimeoutErrorCause;
pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;

#[derive(Debug)]
pub struct RpcTransactionStatusRequest {
    pub transaction_info: TransactionInfo,
    pub wait_until: near_primitives::views::TxExecutionStatus,
}

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

impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "tx_status"
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

impl private::Sealed for RpcTransactionStatusRequest {}
