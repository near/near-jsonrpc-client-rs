//! Sends asynchronous transactions.
//!
//! Sends a signed transaction to the RPC and returns the transaction hash.
//!
//! ## Example
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions};
//! use near_primitives::types::{AccountId, BlockReference};
//! use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
//! use near_crypto::SecretKey;
//! use core::str::FromStr;
//! use serde_json::json;
//! use tokio::time;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let signer_account_id = "fido.testnet".parse::<AccountId>()?;
//! let signer_secret_key = SecretKey::from_str("ed25519:12dhevYshfiRqFSu8DSfxA27pTkmGRv6C5qQWTJYTcBEoB7MSTyidghi5NWXzWqrxCKgxVx97bpXPYQxYN5dieU")?;
//!
//! let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
//!
//! let access_key_query_response = client
//!     .call(methods::query::RpcQueryRequest {
//!         block_reference: BlockReference::latest(),
//!         request: near_primitives::views::QueryRequest::ViewAccessKey {
//!             account_id: signer.account_id.clone(),
//!             public_key: signer.public_key.clone(),
//!         },
//!     })
//!     .await?;
//!
//! let current_nonce = match access_key_query_response.kind {
//!     QueryResponseKind::AccessKey(access_key) => access_key.nonce,
//!     _ => Err("failed to extract current nonce")?,
//!  };
//!     
//! let other_account = "rpc_docs.testnet".parse::<AccountId>()?;
//! let rating = "4.5".parse::<f32>()?;
//!     
//! let transaction = Transaction {
//!     signer_id: signer.account_id.clone(),
//!     public_key: signer.public_key.clone(),
//!     nonce: current_nonce + 1,
//!     receiver_id: "nosedive.testnet".parse::<AccountId>()?,
//!     block_hash: access_key_query_response.block_hash,
//!     actions: vec![Action::FunctionCall(FunctionCallAction {
//!         method_name: "rate".to_string(),
//!         args: json!({
//!             "account_id": other_account,
//!             "rating": rating,
//!         })
//!         .to_string()
//!         .into_bytes(),
//!         gas: 100_000_000_000_000, // 100 TeraGas
//!         deposit: 0,
//!     })],
//! };
//!
//! let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
//!     signed_transaction: transaction.sign(&signer)
//! };
//!
//! let sent_at = time::Instant::now();
//! let tx_hash = client.call(request).await?;
//!
//! loop {
//!     let response = client
//!         .call(methods::tx::RpcTransactionStatusRequest {
//!             transaction_info: transactions::TransactionInfo::TransactionId {
//!                 hash: tx_hash,
//!                 account_id: signer.account_id.clone(),
//!             },
//!         })
//!     .await;
//!     let received_at = time::Instant::now();
//!     let delta = (received_at - sent_at).as_secs();
//!
//!     if delta > 60 {
//!         Err("time limit exceeded for the transaction to be recognized")?;
//!     }
//!
//!     match response {
//!         Err(err) => match err.handler_error()? {
//!             methods::tx::RpcTransactionError::UnknownTransaction { .. } => {
//!                 time::sleep(time::Duration::from_secs(2)).await;
//!                 continue;
//!             }
//!             err => Err(err)?,
//!         },
//!         Ok(response) => {
//!             println!("response gotten after: {}s", delta);
//!             println!("response: {:#?}", response);
//!             break;
//!         }
//!     }
//! }
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
    for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
{
    fn from(this: RpcBroadcastTxAsyncRequest) -> Self {
        Self {
            signed_transaction: this.signed_transaction,
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
