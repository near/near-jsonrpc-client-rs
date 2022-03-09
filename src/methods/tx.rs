//! This module allows you to query the status of a transaction signed by an account_id
//!
//! ## Example
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//! let tx_hash = "B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq".parse()?;
//!
//! let request = methods::tx::RpcTransactionStatusRequest {
//!     transaction_info: methods::tx::TransactionInfo::TransactionId {
//!         hash: tx_hash,
//!         account_id: "itranscend.near".parse()?,
//!    }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert_eq!(tx_hash, response.transaction.hash);
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;

pub type RpcTransactionStatusResponse = near_primitives::views::FinalExecutionOutcomeView;

#[derive(Debug)]
pub struct RpcTransactionStatusRequest {
    pub transaction_info: TransactionInfo,
}

impl From<RpcTransactionStatusRequest>
    for near_jsonrpc_primitives::types::transactions::RpcTransactionStatusCommonRequest
{
    fn from(this: RpcTransactionStatusRequest) -> Self {
        Self {
            transaction_info: this.transaction_info,
        }
    }
}

impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionStatusResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "tx"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(match &self.transaction_info {
            TransactionInfo::Transaction(signed_transaction) => {
                json!([common::serialize_signed_transaction(signed_transaction)?])
            }
            TransactionInfo::TransactionId { hash, account_id } => {
                json!([hash, account_id])
            }
        })
    }
}

impl private::Sealed for RpcTransactionStatusRequest {}
