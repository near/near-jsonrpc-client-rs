use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use near_primitives::transaction::SignedTransaction;

pub type RpcBroadcastTxCommitResponse = near_primitives::views::FinalExecutionOutcomeView;

#[derive(Debug)]
pub struct RpcBroadcastTxCommitRequest {
    pub signed_transaction: SignedTransaction,
}

impl From<RpcBroadcastTxCommitRequest>
    for near_jsonrpc_primitives::types::transactions::RpcSendTransactionRequest
{
    fn from(this: RpcBroadcastTxCommitRequest) -> Self {
        Self {
            signed_transaction: this.signed_transaction,
            wait_until: near_primitives::views::TxExecutionStatus::None,
        }
    }
}

impl RpcMethod for RpcBroadcastTxCommitRequest {
    type Response = RpcBroadcastTxCommitResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "broadcast_tx_commit"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!([common::serialize_signed_transaction(
            &self.signed_transaction
        )?]))
    }
}

impl private::Sealed for RpcBroadcastTxCommitRequest {}
