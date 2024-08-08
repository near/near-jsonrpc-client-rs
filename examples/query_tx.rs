use std::str::FromStr;

use near_jsonrpc_client::methods;
use near_primitives::{hash::CryptoHash, types::AccountId};

mod utils;

pub fn specify_block_reference() -> std::io::Result<near_primitives::types::BlockReference> {
    println!("=========[Block Reference]=========");
    let block_reference = utils::select(
        || {
            println!(" [1] final        \x1b[38;5;244m(alias: f, fin)\x1b[0m");
            println!(" [2] optimistic   \x1b[38;5;244m(alias: o, opt)\x1b[0m");
            println!(" [3] block hash   \x1b[38;5;244m(alias: s, hash)\x1b[0m");
            println!(" [4] block height \x1b[38;5;244m(alias: h, height)\x1b[0m");
        },
        "\x1b[33m(enter a selection)\x1b[0m> ",
        |selection| match (selection, selection.parse()) {
            ("f" | "fin" | "final", _) | (_, Ok(1)) => {
                Some(near_primitives::types::BlockReference::Finality(
                    near_primitives::types::Finality::Final,
                ))
            }
            ("o" | "opt" | "optimistic", _) | (_, Ok(2)) => {
                Some(near_primitives::types::BlockReference::Finality(
                    near_primitives::types::Finality::None,
                ))
            }
            ("s" | "hash" | "block hash", _) | (_, Ok(3)) => loop {
                match utils::input("What block hash should we query? ")
                    .unwrap()
                    .parse()
                {
                    Ok(block_hash) => {
                        break Some(near_primitives::types::BlockReference::BlockId(
                            near_primitives::types::BlockId::Hash(block_hash),
                        ))
                    }
                    _ => println!("(i) Invalid block hash, please reenter!"),
                }
            },
            ("h" | "height" | "block height", _) | (_, Ok(4)) => loop {
                match utils::input("What block height should we query? ")
                    .unwrap()
                    .parse()
                {
                    Ok(block_height) => {
                        break Some(near_primitives::types::BlockReference::BlockId(
                            near_primitives::types::BlockId::Height(block_height),
                        ))
                    }
                    _ => println!("(i) Invalid block height, please reenter!"),
                }
            },
            _ => None,
        },
    )?;
    println!("===================================");

    Ok(block_reference)
}

fn get_valid_input<T: FromStr>(
    prompt: &str,
    max_retries: usize,
) -> Result<T, Box<dyn std::error::Error>> {
    for _ in 0..max_retries {
        let input = utils::input(prompt)?;
        if let Ok(value) = input.parse() {
            return Ok(value);
        } else {
            println!("(i) Invalid input!");
        }
    }

    Err(format!("(i) Maximum number of retries ({}) reached", max_retries).into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = utils::select_network()?;

    // tolerate only 3 retries for a non-failing transaction hash
    for _ in 1..=3 {
        let tx_hash: CryptoHash = get_valid_input("What transaction hash should we query", 3)?;
        let account_id: AccountId = get_valid_input("What account signed this transaction", 3)?;
        let wait_until_str = utils::input("Enter the desired guaranteed execution status (can be one of: NONE, INCLUDED, INCLUDED_FINAL, EXECUTED, FINAL): ")?;
        let wait_until = serde_json::from_value(serde_json::json!(wait_until_str))?;

        match client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: methods::tx::TransactionInfo::TransactionId {
                    tx_hash,
                    sender_account_id: account_id,
                },
                wait_until,
            })
            .await
        {
            Ok(tx_details) => println!("{:#?}", tx_details),
            Err(err) => match err.handler_error() {
                Some(err) => {
                    println!("(i) An error occurred `{:#?}`", err);
                    continue;
                }
                _ => println!("(i) A non-handler error occurred `{:#?}`", err),
            },
        };
        break;
    }

    Ok(())
}
