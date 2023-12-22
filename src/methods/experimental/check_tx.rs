//! Checks a transaction on the network.
//!
//! This code sample doesn't make any request to the RPC node. It's been truncated for brevity sake.
//!
//! An example detailing how to construct a complete request can be found at [`contract_change_method`](https://github.com/near/near-jsonrpc-client-rs/blob/master/examples/contract_change_method.rs).
//!
//! ## Example
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions};
//! use near_primitives::types::{AccountId, BlockReference};
//! use near_primitives::transaction::{Action, Transaction, FunctionCallAction};
//! use near_crypto::SecretKey;
//! use core::str::FromStr;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");
//!
//! let signer_account_id = "fido.testnet".parse::<AccountId>()?;
//! let signer_secret_key = SecretKey::from_str("ed25519:12dhevYshfiRqFSu8DSfxA27pTkmGRv6C5qQWTJYTcBEoB7MSTyidghi5NWXzWqrxCKgxVx97bpXPYQxYN5dieU")?;    // Replace secret_key with valid signer_secret_key
//!
//! let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
//!
//! let other_account = "rpc_docs.testnet".parse::<AccountId>()?;
//! let rating = "4.7".parse::<f32>()?;
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
//! let request = methods::EXPERIMENTAL_check_tx::RpcCheckTxRequest {
//!     signed_transaction: transaction.sign(&signer)
//! };
//! # Ok(())
//! # }
//! ```
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
    for near_jsonrpc_primitives::types::transactions::RpcSendTransactionRequest
{
    fn from(this: RpcCheckTxRequest) -> Self {
        Self {
            signed_transaction: this.signed_transaction,
            wait_until: near_primitives::views::TxExecutionStatus::None,
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
