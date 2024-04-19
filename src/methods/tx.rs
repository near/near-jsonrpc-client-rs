//! Queries the status of a transaction.
//!
//! ## Example
//! Returns the final transaction result for
//! <https://explorer.near.org/transactions/B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq>
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! use near_primitives::views::{FinalExecutionOutcomeViewEnum, TxExecutionStatus};
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//! let tx_hash = "B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq".parse()?;
//!
//! let request = methods::tx::RpcTransactionStatusRequest {
//!     transaction_info: methods::tx::TransactionInfo::TransactionId {
//!         tx_hash,
//!         sender_account_id: "itranscend.near".parse()?,
//!    },
//!     wait_until: TxExecutionStatus::Executed,
//! };
//!
//! let response = client.call(request).await?;
//! let outcome = response.final_execution_outcome.expect("Should be executed by this moment");
//! match outcome {
//!     FinalExecutionOutcomeViewEnum::FinalExecutionOutcome(outcome) => {
//!         assert_eq!(tx_hash, outcome.transaction.hash);
//!     }
//!     FinalExecutionOutcomeViewEnum::FinalExecutionOutcomeWithReceipt(_) => {
//!         panic!("We haven't asked for the receipts");
//!     }
//! };
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use near_jsonrpc_primitives::types::transactions::RpcTransactionResponse;
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
        "tx"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(match &self.transaction_info {
            TransactionInfo::Transaction(signed_transaction) => {
                match signed_transaction {
                    near_jsonrpc_primitives::types::transactions::SignedTransaction::SignedTransaction(tx) => {
                        json!({
                            "signed_tx_base64": common::serialize_signed_transaction(tx)?,
                            "wait_until": self.wait_until
                        })
                    },
                }
            }
            TransactionInfo::TransactionId { tx_hash,sender_account_id } => {
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
