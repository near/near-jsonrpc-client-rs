# near-api-rs (N-AR)

Rust crate for interacting with the NEAR Protocol via RPC API

> DO NOT USE: this crate is unfinalized and therefore, unfit for use.

## Usage

- Using a helper function:

  ```rust
  use near_api_rs::JsonRpcClient;

  // this creates a new client internally, only use this if you only need one client
  // see example 2 in the case where you need multiple clients
  let rpc_client = JsonRpcClient::new_client("http://localhost:3030");

  // The convenience methods on JsonRpcClient aid simplicity
  let status = rpc_client.status().await;

  println!("{:?}", status1);
  ```

- More involved syntax, decoupling method construction and execution and allowing method reuse

  ```rust
  // Here, we manually construct a method and execute that on a client
  // This is useful if you have multiple clients to execute instructions on

  use reqwest::Client;

  use near_api_rs::{RpcMethod, JsonRpcClient};
  use near_primitives::types::AccountId;
  use near_jsonrpc_primitives::views::FinalExecutionOutcomeView;

  let client = reqwest::Client::new(); // instantiate once, share everywhere

  let rpc_client_1 = JsonRpcClient::new("http://localhost:3030", &client);
  let rpc_client_2 = JsonRpcClient::new("http://rpc.website.com", &client);

  let method = RpcMethod::Tx {
      id: "miraclx.near".parse::<AccountId>.unwrap(),
      hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse::<CryptoHash>().unwrap(),
  };

  let tx_status_1: FinalExecutionOutcomeView = method.call_on(&rpc_client_1).await;
  let tx_status_2: FinalExecutionOutcomeView = method.call_on(&rpc_client_2).await;

  println!("{:?}", tx_status_1);
  println!("{:?}", tx_status_2);
  ```

## Testing

- Ensure you have the Rust compiler and package manager installed <https://rustup.rs/>
- Get and initialize the NEAR sandbox <https://github.com/near/sandbox>
- Thereafter;

  ```console
  near-sandbox --home /tmp/near-sandbox init # this happens once ;-)
  near-sandbox --home /tmp/near-sandbox run
  ```

- Execute the test

  ```console
  git clone https://github.com/near/near-api-rs
  cd near-api-rs
  cargo test -- --nocapture
  ```
