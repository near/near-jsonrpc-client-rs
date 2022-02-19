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
