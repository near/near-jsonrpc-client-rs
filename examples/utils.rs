#![allow(unused)]

use near_jsonrpc_client::{auth, JsonRpcClient};
use std::io::{self, Write};

pub fn input(query: &str) -> io::Result<String> {
    print!("{}", query);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn select<S>(print_msg: fn(), query: &str, chk: fn(&str) -> Option<S>) -> io::Result<S> {
    loop {
        print_msg();
        for _ in 1..=5 {
            let selection = input(query)?;
            if let Some(selection) = chk(selection.to_lowercase().as_str()) {
                return Ok(selection);
            }
            println!("\x1b[31m(i)\x1b[0m invalid selection, retry..");
        }
    }
}

pub fn select_network() -> io::Result<JsonRpcClient<auth::Unauthenticated>> {
    println!("========[Network Selection]========");
    let network = select(
        || {
            println!(" [1] mainnet \x1b[38;5;244m(alias: m, main)\x1b[0m");
            println!(" [2] testnet \x1b[38;5;244m(alias: t, test)\x1b[0m");
        },
        "\x1b[33m(enter a selection)\x1b[0m> ",
        |selection| match (selection, selection.parse()) {
            ("m" | "main" | "mainnet", _) | (_, Ok(1)) => Some("mainnet"),
            ("t" | "test" | "testnet", _) | (_, Ok(2)) => Some("testnet"),
            _ => None,
        },
    )?;
    let archival = select(
        || (),
        "Should we connect to an archival node? [y/N] ",
        |selection| match selection {
            "" | "n" | "no" => Some(false),
            "y" | "yes" => Some(true),
            _ => None,
        },
    )?;
    println!(
        "\x1b[32m(i)\x1b[0m Connected to the [{}] network{}",
        network,
        if archival {
            " (via an archival node)"
        } else {
            ""
        }
    );
    let network_url = format!(
        "https://{archival}rpc.{network}.near.org",
        archival = if archival { "archival-" } else { "" },
        network = network
    );
    println!("===================================");

    Ok(JsonRpcClient::connect(&network_url))
}

fn main() {
    panic!("not a binary")
}
