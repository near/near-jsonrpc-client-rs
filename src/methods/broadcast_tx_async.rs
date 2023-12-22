use super::*;

pub use near_primitives::transaction::SignedTransaction;

pub type RpcBroadcastTxAsyncResponse = near_primitives::hash::CryptoHash;

#[derive(Debug)]
pub struct RpcBroadcastTxAsyncRequest {
    pub signed_transaction: SignedTransaction,
}

impl From<RpcBroadcastTxAsyncRequest>
    for near_jsonrpc_primitives::types::transactions::RpcSendTransactionRequest
{
    fn from(this: RpcBroadcastTxAsyncRequest) -> Self {
        Self {
            signed_transaction: this.signed_transaction,
            wait_until: near_primitives::views::TxExecutionStatus::None,
        }
    }
}

#[derive(Debug, Deserialize, Error)]
#[error("{}", unreachable!("fatal: this error should never be constructed"))]
pub enum RpcBroadcastTxAsyncError {}

impl RpcHandlerResponse for RpcBroadcastTxAsyncResponse {}

impl RpcHandlerError for RpcBroadcastTxAsyncError {}

impl RpcMethod for RpcBroadcastTxAsyncRequest {
    type Response = RpcBroadcastTxAsyncResponse;
    type Error = RpcBroadcastTxAsyncError;

    fn method_name(&self) -> &str {
        "broadcast_tx_async"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!([common::serialize_signed_transaction(
            &self.signed_transaction
        )?]))
    }
}

impl private::Sealed for RpcBroadcastTxAsyncRequest {}
