# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `ApiKey::new` now accepts byte arrays and byte slices. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- `ApiKey::as_bytes` returns a byte slice of the key without utf-8 validation. <https://github.com/near/near-jsonrpc-client-rs/pull/119>

### Changed

- `ApiKey::new` no longer requres the input of a valid UUID. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- `Debug` on `ApiKey` doesn't reveal the key anymore. <https://github.com/near/near-jsonrpc-client-rs/pull/120>
- The `auth` module is no longer feature gated. <https://github.com/near/near-jsonrpc-client-rs/pull/119>

### Breaking

- Removed the `auth::IntoApiKey` trait, any thing you can get a byte slice from is now a valid `ApiKey` input. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- Replaced the `ApiKey::as_str` method with `ApiKey::to_str`, now returning a `Result`. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- Replaced the `InvalidApiKey` error with `InvalidHeaderValue` re-exported from `http`. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- Removed `Display` on `ApiKey`. <https://github.com/near/near-jsonrpc-client-rs/pull/117>

## [0.4.1] - 2022-11-11

- Fixed an issue where an `&RpcMethod`'s response was being parsed differently from an `RpcMethod`. <https://github.com/near/near-jsonrpc-client-rs/pull/114>

## [0.4.0] - 2022-10-04

- Updated nearcore dependencies, which now requires a MSRV of `1.64.0`. <https://github.com/near/near-jsonrpc-client-rs/pull/100>, <https://github.com/near/near-jsonrpc-client-rs/pull/110>
- Updated other dependencies, with some general improvements. <https://github.com/near/near-jsonrpc-client-rs/pull/111>
- Added `rustls-tls` feature flag to enable `rustls` as an alternative to `native-tls`. <https://github.com/near/near-jsonrpc-client-rs/pull/103>
- Switched to using `log::debug!` instead of `log::info!` for debug logging. <https://github.com/near/near-jsonrpc-client-rs/pull/107>
- Fixed `gas_price` RPC method serialization. <https://github.com/near/near-jsonrpc-client-rs/pull/73>
- Fixed `query` method error deserialization. <https://github.com/near/near-jsonrpc-client-rs/pull/82>
- Reworked the `JsonRpcError`::`handler_error` method. <https://github.com/near/near-jsonrpc-client-rs/pull/99>
- Moved auth specific logic behind a feature flag. <https://github.com/near/near-jsonrpc-client-rs/pull/55>
- Added the `methods::to_json()` helper method for visualizing the serialization of the RPC methods. <https://github.com/near/near-jsonrpc-client-rs/pull/49>

## [0.4.0-beta.0] - 2022-05-31

<details>
<summary>
  <em>
    Superseded by <a href="https://github.com/near/near-jsonrpc-client-rs/compare/v0.4.0-beta.0...v0.4.0">
      <code> 0.4.0 </code>
    </a>
  </em>
</summary>

> - Updated nearcore dependencies, fixing a previous breaking change. <https://github.com/near/near-jsonrpc-client-rs/pull/100>
> - Fixed `gas_price` RPC method serialization. <https://github.com/near/near-jsonrpc-client-rs/pull/73>
> - Fixed `query` method error deserialization. <https://github.com/near/near-jsonrpc-client-rs/pull/82>
> - Reworked the `JsonRpcError`::`handler_error` method. <https://github.com/near/near-jsonrpc-client-rs/pull/99>
> - Moved auth specific logic behind a feature flag. <https://github.com/near/near-jsonrpc-client-rs/pull/55>
> - Added the `methods::to_json()` helper method for visualizing the serialization of the RPC methods. <https://github.com/near/near-jsonrpc-client-rs/pull/49>

</details>

## [0.3.0] - 2022-02-09

- Dropped generic authentication and added support for custom headers. <https://github.com/near/near-jsonrpc-client-rs/pull/47>
- Added the `sandbox_fast_forward` RPC Method. <https://github.com/near/near-jsonrpc-client-rs/pull/38>
- Upgraded `nearcore` crates to `v0.12.0` <https://github.com/near/near-jsonrpc-client-rs/pull/48>
- Executing the [examples](https://github.com/near/near-jsonrpc-client-rs/tree/master/examples) now allows custom RPC addr specification with interactive server selection. <https://github.com/near/near-jsonrpc-client-rs/commit/b130118d0de806bd9950be306f563559f07c77e6> <https://github.com/near/near-jsonrpc-client-rs/commit/c5e938a90703cb216e99d6f23a43ad9d3812df3d>
- `JsonRpcClient::connect` is now generic over any url-like type. [`Url`](https://docs.rs/url/*/url/struct.Url.html), `&str`, `String` and `&String` are all supported. <https://github.com/near/near-jsonrpc-client-rs/pull/35>
- `JsonRpcClient` now defaults to the `Unauthenticated` state, easing a type specification pain point. <https://github.com/near/near-jsonrpc-client-rs/pull/36>

## [0.2.0] - 2021-12-22

- Updated nearcore version to `0.11.0` (<https://github.com/near/nearcore/pull/5943>).
- Fixed `chunk` method serialization. <https://github.com/near/near-jsonrpc-client-rs/commit/f40ad743653ad2a1a9eb5eaa96c302c9b531bf40>
- `Client::call` no longer consumes the client. <https://github.com/near/near-jsonrpc-client-rs/commit/471a53be062e0880c6bc5c2721d123da2a9e0c2e>
- Implemented workaround for partially serialized server responses. <https://github.com/near/near-jsonrpc-client-rs/pull/29>
- Dropped base64 API token support in favor of a generic key-value approach. <https://github.com/near/near-jsonrpc-client-rs/commit/dd7761b51e1775350be1782370aa22c0b0fe98d7>
- Added examples to repo. <https://github.com/near/near-jsonrpc-client-rs/pull/32>
- Ensured `None`-typed wrapped errors are actual errors (i.e. have all traits attributed with errors, especially `fmt::Display`). <https://github.com/near/near-jsonrpc-client-rs/pull/34>

## [0.1.0] - 2021-11-11

> Release Page: <https://github.com/near/near-jsonrpc-client-rs/releases/tag/v0.1.0>

[unreleased]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.3.0...v0.4.0
[0.4.0-beta.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.3.0...v0.4.0-beta.0
[0.3.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/near/near-jsonrpc-client-rs/releases/tag/v0.1.0
