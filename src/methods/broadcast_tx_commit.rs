//! Sends blocking transactions.
//!
//! Sends a signed transaction to the RPC and waits until the transaction is fully complete.
//!
//! Constructs a signed transaction to be sent to an RPC node.
//!
//! This code sample doesn't make any requests to the RPC node. It only shows how to construct the request. It's been truncated for brevity.
//!
//! A full example on how to use `broadcast_tx_commit` method can be found at [`contract_change_method`](https://github.com/near/near-jsonrpc-client-rs/blob/master/examples/contract_change_method_commit.rs).
//!
//! ## Example
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions::TransactionInfo};
//! use near_primitives::types::{AccountId, BlockReference};
//! use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let signer_account_id = "fido.testnet".parse::<AccountId>()?;
//! let signer_secret_key = "ed25519:12dhevYshfiRqFSu8DSfxA27pTkmGRv6C5qQWTJYTcBEoB7MSTyidghi5NWXzWqrxCKgxVx97bpXPYQxYN5dieU".parse()?;
//!
//! let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
//!     
//! let other_account = "rpc_docs.testnet".parse::<AccountId>()?;
//! let rating = "4.5".parse::<f32>()?;
//!     
//! let transaction = Transaction {
//!     signer_id: signer.account_id.clone(),
//!     public_key: signer.public_key.clone(),
//!     nonce: 904565 + 1,
//!     receiver_id: "nosedive.testnet".parse::<AccountId>()?,
//!     block_hash: "AUDcb2iNUbsmCsmYGfGuKzyXKimiNcCZjBKTVsbZGnoH".parse()?,
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
//! let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
//!     signed_transaction: transaction.sign(&signer)
//! };
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use near_primitives::transaction::SignedTransaction;

pub type RpcBroadcastTxCommitResponse = near_primitives::views::FinalExecutionOutcomeView;

#[derive(Debug)]
pub struct RpcBroadcastTxCommitRequest {
    pub signed_transaction: SignedTransaction,
}

impl From<RpcBroadcastTxCommitRequest>
    for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
{
    fn from(this: RpcBroadcastTxCommitRequest) -> Self {
        Self {
            signed_transaction: this.signed_transaction,
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
