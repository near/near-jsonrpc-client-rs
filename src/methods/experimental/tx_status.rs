use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;

pub type RpcTransactionStatusResponse =
    near_primitives::views::FinalExecutionOutcomeWithReceiptView;

#[derive(Debug)]
pub struct RpcTransactionStatusRequest {
    pub transaction_info: TransactionInfo,
}

impl From<RpcTransactionStatusRequest>
    for near_jsonrpc_primitives::types::transactions::RpcTransactionStatusRequest
{
    fn from(this: RpcTransactionStatusRequest) -> Self {
        Self {
            transaction_info: this.transaction_info,
            wait_until: near_primitives::views::TxExecutionStatus::None,
        }
    }
}

impl RpcHandlerResponse for RpcTransactionStatusResponse {}

impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionStatusResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_tx_status"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(match &self.transaction_info {
            TransactionInfo::Transaction(signed_transaction) => {
                match signed_transaction {
                    near_jsonrpc_primitives::types::transactions::SignedTransaction::SignedTransaction(tx) => {
                        json!([common::serialize_signed_transaction(tx)?])
                    },
                }
            }
            TransactionInfo::TransactionId { tx_hash,sender_account_id } => {
                json!([tx_hash, sender_account_id])
            }
        })
    }
}

impl private::Sealed for RpcTransactionStatusRequest {}
