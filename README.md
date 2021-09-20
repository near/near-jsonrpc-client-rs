# near-jsonrpc-client

Lower-level JSON RPC API for interfacing with the NEAR Protocol.

It's recommended to use the higher-level `near-api` library instead. Rust version coming soon.

> DO NOT USE: this crate is unfinalized and therefore, unfit for use.

## Usage

Each one of the valid JSON RPC methods are defined in the `methods` module.
For instance, to make a `tx` request, you start with the `tx` module
and construct a request using the `methods::tx::RpcTransactionStatusRequest` struct.

```rust
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::transactions::TransactionInfo;

// create a client and connect to a NEAR JSON-RPC server
let mainnet_client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");

let tx_status_request = methods::tx::RpcTransactionStatusRequest {
    transaction_info: TransactionInfo::TransactionId {
        hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse()?,
        account_id: "miraclx.near".parse()?,
    },
};

// call a method on the server via the connected client
let tx_status = mainnet_client.call(&tx_status_request).await?;

println!("{:?}", tx_status);
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
