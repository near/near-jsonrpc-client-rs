//! Sends asynchronous transactions.
//!
//! ## Example
//!
//! Constructs a signed transaction to be sent to an RPC node. It returns the transaction hash if successful.
//!
//! This code sample doesn't make any requests to the RPC node. It only shows how to construct the request. It's been truncated for brevity sake.
//!
//! A full example on how to use `broadcast_tx_async` method can be found at [`contract_change_method`](https://github.com/near/near-jsonrpc-client-rs/blob/master/examples/contract_change_method.rs).
//!
//! ```no_run
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::gas::Gas;
//! use near_primitives::types::{AccountId, Balance};
//! use near_primitives::transaction::{Action, FunctionCallAction, Transaction, TransactionV0};
//! use near_crypto::SecretKey;
//! use core::str::FromStr;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.fastnear.com");
//!
//! let signer_account_id = "fido.testnet".parse::<AccountId>()?;
//! let signer_secret_key = SecretKey::from_str("ed25519:12dhevYshfiRqFSu8DSfxA27pTkmGRv6C5qQWTJYTcBEoB7MSTyidghi5NWXzWqrxCKgxVx97bpXPYQxYN5dieU")?;
//!
//! let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
//!
//! let other_account = "rpc_docs.testnet".parse::<AccountId>()?;
//! let rating = "4.5".parse::<f32>()?;
//!
//! let transaction = Transaction::V0(TransactionV0 {
//!     signer_id: signer.get_account_id(),
//!     public_key: signer.public_key().clone(),
//!     nonce: 10223934 + 1,
//!     receiver_id: "nosedive.testnet".parse::<AccountId>()?,
//!     block_hash: "AUDcb2iNUbsmCsmYGfGuKzyXKimiNcCZjBKTVsbZGnoH".parse()?,
//!     actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
//!         method_name: "rate".to_string(),
//!         args: json!({
//!             "account_id": other_account,
//!             "rating": rating,
//!         })
//!         .to_string()
//!         .into_bytes(),
//!         gas: Gas::from_teragas(100),
//!         deposit: Balance::ZERO,
//!     }))],
//! });
//!
//! let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
//!     signed_transaction: transaction.sign(&signer)
//! };
//! # Ok(())
//! # }
//! ```
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

#[derive(Debug, Serialize, Deserialize, Error)]
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
