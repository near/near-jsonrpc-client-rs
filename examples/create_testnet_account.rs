use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_jsonrpc_primitives::types::transactions::TransactionInfo;
use near_primitives::hash::CryptoHash;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::{AccountId, BlockReference};

use serde_json::json;
use tokio::time;

mod utils;

async fn get_current_nonce(
    client: &JsonRpcClient,
    account_id: &AccountId,
    public_key: &near_crypto::PublicKey,
) -> Result<(CryptoHash, u64), Box<dyn std::error::Error>> {
    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: account_id.clone(),
                public_key: public_key.clone(),
            },
        })
        .await?;

    match access_key_query_response.kind {
        QueryResponseKind::AccessKey(access_key) => {
            Ok((access_key_query_response.block_hash, access_key.nonce))
        }
        _ => Err("failed to extract current nonce")?,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    let signer_account_id = utils::input("Enter the creators Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the creators's private key: ")?.parse()?;

    let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);

    let (latest_hash, current_nonce) =
        get_current_nonce(&client, &signer.account_id, &signer.public_key).await?;

    let new_account_id = utils::input("What's the new Account ID: ")?;
    let mut initial_deposit = None;
    while let None = initial_deposit {
        match utils::input("How much do you want to fund this account with (in Ⓝ units)? ")?
            .parse()?
        {
            deposit @ 1.. => {
                initial_deposit.replace(deposit);
            }
            _ => {}
        }
    }

    let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);

    println!("=====================================================");
    println!("New Account ID: {}", new_account_id);
    println!("    Secret Key: {}", new_key_pair);
    println!("    Public Key: {}", new_key_pair.public_key());

    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: "testnet".parse()?,
        block_hash: latest_hash,
        actions: vec![Action::FunctionCall(FunctionCallAction {
            method_name: "create_account".to_string(),
            args: json!({
                "new_account_id": new_account_id,
                "new_public_key": new_key_pair.public_key(),
            })
            .to_string()
            .into_bytes(),
            gas: 300_000_000_000_000,
            deposit: initial_deposit.unwrap_or(1) * 1_000_000_000_000_000_000_000_000,
        })],
    };

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: transaction.sign(&signer),
    };

    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await?;

    println!("       Tx Hash: {}", tx_hash);
    println!("=====================================================");

    loop {
        let response = client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: TransactionInfo::TransactionId {
                    hash: tx_hash,
                    account_id: signer.account_id.clone(),
                },
            })
            .await;
        let received_at = time::Instant::now();
        let delta = (received_at - sent_at).as_secs();

        if delta > 60 {
            Err("time limit exceeded for the transaction to be recognized")?;
        }

        match response {
            Err(err) => match err.handler_error()? {
                methods::tx::RpcTransactionError::UnknownTransaction { .. } => {
                    time::sleep(time::Duration::from_secs(2)).await;
                    continue;
                }
                err => Err(err)?,
            },
            Ok(outcome) => {
                // outcome.status == SuccessValue(`true`)
                if matches!(outcome.status, near_primitives::views::FinalExecutionStatus::SuccessValue(ref s) if s == "dHJ1ZQ==")
                {
                    println!("Account successfully created after {}s", delta);
                } else {
                    println!("{:#?}", outcome);
                    println!("Creating the account failed, check above for full logs");
                }
                break;
            }
        }
    }

    Ok(())
}
