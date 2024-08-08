//! Creates an account on the network.
//!
//! Creates either;
//! - a top-level mainnet / testnet account
//! - or a sub-account for any account on the network.
//!
//! top-level account example: `miraclx.near` creates `foobar.near`
//! sub-account example: `miraclx.near` creates `test.miraclx.near`
//!
//! This script is interactive.

use near_crypto::Signer;
use near_jsonrpc_client::methods::broadcast_tx_commit::RpcTransactionError;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::{
    Action, AddKeyAction, CreateAccountAction, FunctionCallAction, Transaction, TransactionV0,
    TransferAction,
};
use near_primitives::types::{AccountId, BlockReference};
use near_primitives::views::{FinalExecutionStatus, TxExecutionStatus};

use serde_json::json;
use tokio::time;

mod utils;

async fn account_exists(
    client: &JsonRpcClient,
    account_id: &AccountId,
) -> Result<bool, Box<dyn std::error::Error>> {
    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccount {
                account_id: account_id.clone(),
            },
        })
        .await;

    match access_key_query_response {
        Ok(_) => Ok(true),
        Err(near_jsonrpc_client::errors::JsonRpcError::ServerError(
            near_jsonrpc_client::errors::JsonRpcServerError::HandlerError(
                near_jsonrpc_primitives::types::query::RpcQueryError::UnknownAccount { .. },
            ),
        )) => Ok(false),
        Err(res) => Err(res)?,
    }
}

async fn get_current_nonce(
    client: &JsonRpcClient,
    account_id: &AccountId,
    public_key: &near_crypto::PublicKey,
) -> Result<Option<(CryptoHash, u64)>, Box<dyn std::error::Error>> {
    let query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: account_id.clone(),
                public_key: public_key.clone(),
            },
        })
        .await;

    match query_response {
        Ok(access_key_query_response) => match access_key_query_response.kind {
            QueryResponseKind::AccessKey(access_key) => Ok(Some((
                access_key_query_response.block_hash,
                access_key.nonce,
            ))),
            _ => Err("failed to extract current nonce")?,
        },
        Err(near_jsonrpc_client::errors::JsonRpcError::ServerError(
            near_jsonrpc_client::errors::JsonRpcServerError::HandlerError(
                near_jsonrpc_primitives::types::query::RpcQueryError::UnknownAccessKey { .. },
            ),
        )) => Ok(None),
        Err(res) => Err(res)?,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = utils::select_network()?;

    let signer_account_id = loop {
        let signer_account_id = utils::input("Enter the creators Account ID: ")?.parse()?;
        if account_exists(&client, &signer_account_id).await? {
            break signer_account_id;
        }
        println!("(i) This account doesn't exist, please reenter!");
    };

    let (signer, latest_hash, current_nonce) = loop {
        let signer_secret_key = utils::input("Enter the creators's private key: ")?.parse()?;

        let signer = near_crypto::InMemorySigner::from_secret_key(
            signer_account_id.clone(),
            signer_secret_key,
        );

        if let Some((latest_hash, current_nonce)) =
            get_current_nonce(&client, &signer.account_id, &signer.public_key).await?
        {
            break (signer, latest_hash, current_nonce);
        }
        println!("(i) Invalid access key, please reenter!");
    };

    let new_account_id = loop {
        let new_account_id = utils::input("What's the new Account ID: ")?.parse()?;
        if !account_exists(&client, &new_account_id).await? {
            break new_account_id;
        }
        println!("(i) This account already exists, please reenter!");
    };

    let initial_deposit = loop {
        let deposit: f64 =
            utils::input("How much do you want to fund this account with (in Ⓝ units)? ")?
                .parse()?;
        if deposit >= 0.0 {
            break ((deposit * 1_000_000.0) as u128) * 1_000_000_000_000_000_000_u128;
        }
        println!("(i) Enter a non-zero deposit value!");
    };

    let is_sub_account = new_account_id.is_sub_account_of(&signer.account_id);
    let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);

    let (transaction, expected_output) = if is_sub_account {
        (
            TransactionV0 {
                signer_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
                nonce: current_nonce + 1,
                receiver_id: new_account_id.clone(),
                block_hash: latest_hash,
                actions: vec![
                    Action::CreateAccount(CreateAccountAction {}),
                    Action::AddKey(Box::new(AddKeyAction {
                        access_key: near_primitives::account::AccessKey {
                            nonce: 0,
                            permission: near_primitives::account::AccessKeyPermission::FullAccess,
                        },
                        public_key: new_key_pair.public_key(),
                    })),
                    Action::Transfer(TransferAction {
                        deposit: initial_deposit,
                    }),
                ],
            },
            vec![],
        )
    } else {
        let contract_id = if client.server_addr().ends_with("testnet.near.org") {
            "testnet".parse()?
        } else if client.server_addr().ends_with("mainnet.near.org") {
            "near".parse()?
        } else {
            Err("can only create non-sub accounts for mainnet / testnet\nconsider creating a sub-account instead")?
        };
        (
            TransactionV0 {
                signer_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
                nonce: current_nonce + 1,
                receiver_id: contract_id,
                block_hash: latest_hash,
                actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
                    method_name: "create_account".to_string(),
                    args: json!({
                        "new_account_id": new_account_id,
                        "new_public_key": new_key_pair.public_key(),
                    })
                    .to_string()
                    .into_bytes(),
                    gas: 300_000_000_000_000,
                    deposit: initial_deposit,
                }))],
            },
            b"true".to_vec(),
        )
    };

    println!("=============================================================");
    println!("New Account ID: {}", new_account_id);
    println!("    Secret Key: {}", new_key_pair);
    println!("    Public Key: {}", new_key_pair.public_key());
    println!("       Deposit: {}", initial_deposit);
    println!("-------------------------------------------------------------");

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: Transaction::V0(transaction).sign(&Signer::InMemory(signer.clone())),
    };

    let sent_at = time::Instant::now();

    let tx_hash = client.call(request).await?;

    println!("       Tx Hash: {}", tx_hash);
    println!("=============================================================");

    loop {
        let response = client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: TransactionInfo::TransactionId {
                    tx_hash,
                    sender_account_id: signer.account_id.clone(),
                },
                wait_until: TxExecutionStatus::Final,
            })
            .await;
        let received_at = time::Instant::now();
        let delta = (received_at - sent_at).as_secs();

        if delta > 60 {
            Err("time limit exceeded for the transaction to be recognized")?;
        }

        match response {
            Ok(tx) => {
                // it's fine to unwrap because we asked for finalized tx
                let outcome = tx.final_execution_outcome.unwrap().into_outcome();
                match outcome.status {
                    FinalExecutionStatus::Failure(err) => {
                        println!("{:#?}", err);
                        println!("(!) Creating the account failed, check above for full logs");
                        break;
                    }
                    FinalExecutionStatus::SuccessValue(ref s) => {
                        if s == &expected_output {
                            println!("(i) Account successfully created after {}s", delta);
                        } else {
                            println!("{:#?}", outcome);
                            println!("(!) Creating the account failed, check above for full logs");
                        }
                        break;
                    }
                    _ => {}
                }
            }
            Err(err) => match err.handler_error() {
                Some(
                    RpcTransactionError::TimeoutError
                    | RpcTransactionError::UnknownTransaction { .. },
                ) => {
                    time::sleep(time::Duration::from_secs(2)).await;
                    continue;
                }
                _ => Err(err)?,
            },
        }
    }

    Ok(())
}
