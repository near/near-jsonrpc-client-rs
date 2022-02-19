use super::*;

pub use near_jsonrpc_primitives::types::transactions::{
    RpcBroadcastTxSyncResponse, RpcTransactionError,
};
pub use near_primitives::transaction::SignedTransaction;

#[derive(Debug)]
pub struct RpcCheckTxRequest {
    pub signed_transaction: SignedTransaction,
}

impl From<RpcCheckTxRequest>
    for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
{
    fn from(this: RpcCheckTxRequest) -> Self {
        Self {
            signed_transaction: this.signed_transaction,
        }
    }
}

impl RpcMethod for RpcCheckTxRequest {
    type Response = RpcBroadcastTxSyncResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_check_tx"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!([common::serialize_signed_transaction(
            &self.signed_transaction
        )?]))
    }
}

impl private::Sealed for RpcCheckTxRequest {}
