# near-jsonrpc-client

Generic, low-level interfaces for interacting with the NEAR Protocol via JSON_RPC / HTTP.

> DO NOT USE: this crate is unfinalized and therefore, unfit for use.

## Usage

- Calling JSON_RPC/HTTP methods:

  ```rust
  use near_jsonrpc_client::NearClient;

  // creates a generic JSON_RPC/HTTP NEAR Client
  let near_client = NearClient::new().connect("http://localhost:3030");

  // creates an JSON_RPC interface based off the existing NEAR Client
  let jsonrpc_client = near_client.as_jsonrpc();

  // creates an HTTP interface based off the existing NEAR Client
  let http_client = near_client.as_http();

  // The convenience methods on NearJsonRpcClient aid simplicity
  let status1 = jsonrpc_client.status().await?;

  // The convenience methods on NearHttpClient aid simplicity
  let status2 = http_client.status().await?;

  println!("{:?}", status1);
  println!("{:?}", status2);
  ```

- More involved syntax, decoupling method construction, execution and allowing method reuse

  ```rust
  // Here, we manually construct a method and execute that on a client
  // This is useful if you have multiple clients to call methods on

  use near_jsonrpc_client::{jsonrpc::JsonRpcMethod, NearClient};
  use near_jsonrpc_client::{NEAR_MAINNET_RPC_URL, NEAR_TESTNET_RPC_URL};
  use near_jsonrpc_primitives::views::FinalExecutionOutcomeView;
  use near_primitives::types::AccountId;

  let client_builder = NearClient::new(); // instantiate once, reuse

  let mainnet_jsonrpc_client = client_builder.connect(NEAR_MAINNET_RPC_URL).as_jsonrpc();
  let testnet_jsonrpc_client = client_builder.connect(NEAR_TESTNET_RPC_URL).as_jsonrpc();

  let method = RpcMethod::Tx {
      // this method can be reused
      id: "miraclx.near".parse::<AccountId>()?,
      hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse::<CryptoHash>()?,
  };

  let tx_status_on_mainnet: FinalExecutionOutcomeView =
      method.call_on(&mainnet_jsonrpc_client).await?;
  let tx_status_on_testnet: FinalExecutionOutcomeView =
      method.call_on(&testnet_jsonrpc_client).await?;

  println!("{:?}", tx_status_on_mainnet);
  println!("{:?}", tx_status_on_testnet);
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
  git clone https://github.com/near/near-jsonrpc-client-rs
  cd near-jsonrpc-client-rs
  cargo test -- --nocapture
  ```
