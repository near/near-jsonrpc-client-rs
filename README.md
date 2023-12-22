# near-jsonrpc-client

Lower-level API for interfacing with the NEAR Protocol via JSONRPC.

[![Crates.io](https://img.shields.io/crates/v/near-jsonrpc-client?label=latest)](https://crates.io/crates/near-jsonrpc-client)
[![Documentation](https://docs.rs/near-jsonrpc-client/badge.svg)](https://docs.rs/near-jsonrpc-client)
[![MIT or Apache 2.0 Licensed](https://img.shields.io/crates/l/near-jsonrpc-client.svg)](#license)
[![Dependency Status](https://deps.rs/crate/near-jsonrpc-client/0.5.1/status.svg)](https://deps.rs/crate/near-jsonrpc-client/0.5.1)

## Usage

Each one of the valid JSON RPC methods are defined in the `methods` module.
For instance, to make a `tx` request, you start with the `tx` module
and construct a request using the `methods::tx::RpcTransactionStatusRequest` struct.

```rust
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::transactions::TransactionInfo;

let mainnet_client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");

let tx_status_request = methods::tx::RpcTransactionStatusRequest {
    transaction_info: TransactionInfo::TransactionId {
        hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse()?,
        account_id: "miraclx.near".parse()?,
    },
};

// call a method on the server via the connected client
let tx_status = mainnet_client.call(tx_status_request).await?;

println!("{:?}", tx_status);
```

Check out [`the examples folder`](https://github.com/near/near-jsonrpc-client-rs/tree/master/examples) for a comprehensive list of helpful demos. You can run the examples with `cargo`. For example: `cargo run --example view_account`.


## Releasing

Versioning and releasing of this crate is automated and managed by [custom fork](https://github.com/miraclx/cargo-workspaces/tree/grouping-versioning-and-exclusion) of [`cargo-workspaces`](https://github.com/pksunkara/cargo-workspaces). To publish a new version of this crate, you can do so by bumping the `version` under the `[workspace.metadata.workspaces]` section in the [package manifest](https://github.com/near/near-jsonrpc-client-rs/blob/master/Cargo.toml) and submit a PR.

We have CI Infrastructure put in place to automate the process of publishing all crates once a version change has merged into master.

However, before you release, make sure the [CHANGELOG](https://github.com/near/near-jsonrpc-client-rs/blob/master/CHANGELOG.md) is up to date and that the `[Unreleased]` section is present but empty.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
