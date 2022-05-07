#![allow(unused)]

use near_jsonrpc_client::JsonRpcClient;
use std::io::{self, Write};

pub fn input(query: &str) -> io::Result<String> {
    print!("{}", query);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

pub fn select<S, F>(print_msg: fn(), query: &str, chk: F) -> io::Result<S>
where
    F: Fn(&str) -> Option<S>,
{
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

pub fn select_network() -> io::Result<JsonRpcClient> {
    println!("========[Network Selection]========");
    let network = select(
        || {
            println!(" [1] mainnet \x1b[38;5;244m(alias: m, main)\x1b[0m");
            println!(" [2] testnet \x1b[38;5;244m(alias: t, test)\x1b[0m");
            println!(" [3] custom  \x1b[38;5;244m(alias:       c)\x1b[0m");
        },
        "\x1b[33m(enter a selection)\x1b[0m> ",
        |selection| match (selection, selection.parse()) {
            ("m" | "main" | "mainnet", _) | (_, Ok(1)) => Some("mainnet"),
            ("t" | "test" | "testnet", _) | (_, Ok(2)) => Some("testnet"),
            ("c" | "custom", _) | (_, Ok(3)) => Some("custom"),
            _ => None,
        },
    )?;
    let network_url = if network != "custom" {
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
        format!(
            "https://{archival}rpc.{network}.near.org",
            archival = if archival { "archival-" } else { "" },
            network = network
        )
    } else {
        loop {
            let url = input("Enter the RPC Server Address: ")?;
            if let Err(err) = url.parse::<reqwest::Url>() {
                println!("\x1b[31m(i)\x1b[0m invalid url ({}), retry..", err);
                continue;
            }
            break url;
        }
    };
    println!("===================================");

    Ok(JsonRpcClient::connect(network_url))
}

fn main() {
    panic!("not a binary")
}
