# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.5.1...v0.6.0) - 2023-06-02

### Added
- add logging ([#97](https://github.com/near/near-jsonrpc-client-rs/pull/97))
- allow floating point deposits with create-account example ([#95](https://github.com/near/near-jsonrpc-client-rs/pull/95))
- *(examples)* contract_change_method example using the `broadcast_tx_commit` RPC method ([#86](https://github.com/near/near-jsonrpc-client-rs/pull/86))
- add ability to serialize requests to preview server request payload ([#49](https://github.com/near/near-jsonrpc-client-rs/pull/49))
- *(sandbox)* impl client support for fast forwarding ([#38](https://github.com/near/near-jsonrpc-client-rs/pull/38))
- custom headers ([#47](https://github.com/near/near-jsonrpc-client-rs/pull/47))
- *(infra)* introduce automated publishing ([#41](https://github.com/near/near-jsonrpc-client-rs/pull/41))
- JsonRpcClient, default to Unauthenticated state ([#36](https://github.com/near/near-jsonrpc-client-rs/pull/36))
- connect with generic AsUrl ([#35](https://github.com/near/near-jsonrpc-client-rs/pull/35))
- make auth interface more generic ([#26](https://github.com/near/near-jsonrpc-client-rs/pull/26))
- add support for basic authentication tokens ([#18](https://github.com/near/near-jsonrpc-client-rs/pull/18))
- impl smarter Any result descriptors ([#15](https://github.com/near/near-jsonrpc-client-rs/pull/15))
- allow construction of generic method requests ([#13](https://github.com/near/near-jsonrpc-client-rs/pull/13))

### Fixed
- incorrect log level ([#107](https://github.com/near/near-jsonrpc-client-rs/pull/107))
- rework the JsonRpcError::handler_error method ([#99](https://github.com/near/near-jsonrpc-client-rs/pull/99))
- unserialized error compat is for `.error_struct`, not `.data` ([#94](https://github.com/near/near-jsonrpc-client-rs/pull/94))
- fix examples network selector by index ([#93](https://github.com/near/near-jsonrpc-client-rs/pull/93))
- patch query response deserialization, add tests ([#82](https://github.com/near/near-jsonrpc-client-rs/pull/82))
- `gas_price` RPC method serialization ([#73](https://github.com/near/near-jsonrpc-client-rs/pull/73))
- fix typo :sigh
- mitigate deserialization of partially serialized error types ([#29](https://github.com/near/near-jsonrpc-client-rs/pull/29))
- fix serialization of methods::chunk payload
- call should not consume the client
- fix doc compilation without the any feature flag
- fix test with updated call_on syntax
- fix chunk method call with #[serde(untagged)]
- fix signature on http methods, non-static
- fix license to Apache-2.0 to match Cargo.toml

### Other
- Use release-plz instead of cargo-workspaces to manage releases straightforwardly ([#127](https://github.com/near/near-jsonrpc-client-rs/pull/127))
- Upgrade near primitive crates version to 0.17.0 ([#126](https://github.com/near/near-jsonrpc-client-rs/pull/126))
- version to 0.5.1 ([#125](https://github.com/near/near-jsonrpc-client-rs/pull/125))
- bump borsh to 0.10 ([#124](https://github.com/near/near-jsonrpc-client-rs/pull/124))
- version to 0.5.0 ([#123](https://github.com/near/near-jsonrpc-client-rs/pull/123))
- updated near-primitives and associated crates ([#122](https://github.com/near/near-jsonrpc-client-rs/pull/122))
- introduce Authorization::bearer helper method ([#121](https://github.com/near/near-jsonrpc-client-rs/pull/121))
- *(docs)* document the `auth` module ([#85](https://github.com/near/near-jsonrpc-client-rs/pull/85))
- `ApiKey` type can be debugged without exposing the api key ([#120](https://github.com/near/near-jsonrpc-client-rs/pull/120))
- remove uuid api key requirement ([#119](https://github.com/near/near-jsonrpc-client-rs/pull/119))
- update changelog
- deprecate `Display` impl for `ApiKey` ([#117](https://github.com/near/near-jsonrpc-client-rs/pull/117))
- update changelog
- &RpcMethod response handling should match RpcMethod's ([#114](https://github.com/near/near-jsonrpc-client-rs/pull/114))
- highlight the contributors in every release ([#113](https://github.com/near/near-jsonrpc-client-rs/pull/113))
- version to 0.4.0 ([#112](https://github.com/near/near-jsonrpc-client-rs/pull/112))
- update deps ([#111](https://github.com/near/near-jsonrpc-client-rs/pull/111))
- update to 0.15.0 of NEAR crates ([#110](https://github.com/near/near-jsonrpc-client-rs/pull/110))
- Add "rustls-tls" feature flag to enable rustls-tls feature in reqwest instead of native-tls. ([#103](https://github.com/near/near-jsonrpc-client-rs/pull/103))
- version to `0.4.0-beta.0` ([#101](https://github.com/near/near-jsonrpc-client-rs/pull/101))
- update nearcore dependencies ([#100](https://github.com/near/near-jsonrpc-client-rs/pull/100))
- add query tx example ([#98](https://github.com/near/near-jsonrpc-client-rs/pull/98))
- parse a Message to serde_json::Value intermediately
- extend query block example to support custom block references ([#96](https://github.com/near/near-jsonrpc-client-rs/pull/96))
- extend create account functionality to support sub-accounts
- add example for creating a top-level testnet account
- *(docs)* document the `health` RPC method ([#61](https://github.com/near/near-jsonrpc-client-rs/pull/61))
- *(docs)* document the `status` RPC method ([#58](https://github.com/near/near-jsonrpc-client-rs/pull/58))
- *(docs)* document the `light_client_proof` RPC method ([#77](https://github.com/near/near-jsonrpc-client-rs/pull/77))
- *(docs)* document the `network_info` RPC method ([#59](https://github.com/near/near-jsonrpc-client-rs/pull/59))
- update auth urls in example to match dev console
- update changelog
- *(docs)* document the `chunk` RPC method ([#62](https://github.com/near/near-jsonrpc-client-rs/pull/62))
- *(docs)* document the `block` RPC method ([#60](https://github.com/near/near-jsonrpc-client-rs/pull/60))
- *(docs)* document `tx` jsonrpc method ([#53](https://github.com/near/near-jsonrpc-client-rs/pull/53))
- *(CI)* drop rustdoc pipeline ([#56](https://github.com/near/near-jsonrpc-client-rs/pull/56))
- move auth-specific logic behind the default `auth` feature flag ([#55](https://github.com/near/near-jsonrpc-client-rs/pull/55))
- document the `any` helper and `sandbox` RPC methods ([#54](https://github.com/near/near-jsonrpc-client-rs/pull/54))
- decouple methods into individual fs modules ([#50](https://github.com/near/near-jsonrpc-client-rs/pull/50))
- version to 0.3.0 ([#44](https://github.com/near/near-jsonrpc-client-rs/pull/44))
- upgrade nearcore crates to 0.12.0
- :ValidRpcMarkerTrait -> private::Sealed
- treat reqwest::Url as a Client connectable Url
- apply clippy lints ([#43](https://github.com/near/near-jsonrpc-client-rs/pull/43))
- add CHANGELOG file ([#39](https://github.com/near/near-jsonrpc-client-rs/pull/39))
- *(ci)* skip rust installation in rustdoc pipeline ([#40](https://github.com/near/near-jsonrpc-client-rs/pull/40))
- support custom rpc addr
- allow network selection
- Update README.md
- add examples to repo ([#32](https://github.com/near/near-jsonrpc-client-rs/pull/32))
- bump crate version to 0.2.0 ([#33](https://github.com/near/near-jsonrpc-client-rs/pull/33))
- ensure None-typed errors are actual errors (displayable, etc.) ([#34](https://github.com/near/near-jsonrpc-client-rs/pull/34))
- dependencies updated
- drop base64 auth key support, impl generic apikey token input
- add badges to readme
- license to MIT + Apache-2.0
- use published nearcore versions
- [**breaking**] impl statefully typed API design around authentication ([#22](https://github.com/near/near-jsonrpc-client-rs/pull/22))
- Upgrade to MSRV 1.56 + 2021 edition ([#21](https://github.com/near/near-jsonrpc-client-rs/pull/21))
- remove rust-toolchain
- update nearcore + dump actix_derive pin
- return raw RpcError in non-contextual cases
- export RpcAnyRequest to the public api
- move methods referencing self a little higher for clarity
- drop manual Error impl
- drop testing sectio from readme
- conditionally impl RpcHandler{Response,Error} for serde_json::Value
- add server_addr method for retrieving the inner server_addr of a client
- add typed/untyped tests for the methods::any() generic constructor
- impl HandlerResult + HandlerError for serde_json::Value
- use Rpc-* aliases for non-Rpc-* reexports
- drop default RpcMethod params for clarity
- use *-Response aliases for non-Response reexports
- hide `(): RpcHandlerResponse` behind adversarial flag
- rename terminology: request -> response
- drop legacy after introduction of `methods::any()`
- cleanup crate in preparedness for release ([#14](https://github.com/near/near-jsonrpc-client-rs/pull/14))
- impl From<RpcError> for JsonRpcError
- add conditional support for legacy behaviours: block_by_id + query_by_path
- skip activating the adversarial flag on near-jsonrpc-primitives
- update nearcore; fix adversarial feat build
- *(methods)* promote use of traits, double down on the macro complexity ([#11](https://github.com/near/near-jsonrpc-client-rs/pull/11))
- owned returns from handler_error
- add handler_method on JsonRpcError to aid selecting the handler err directly
- refactor to allow parsing the `.data` field in err response if `.error_struct fails
- reintroduce EXPERIMENTAL_tx_status ([#10](https://github.com/near/near-jsonrpc-client-rs/pull/10))
- update nearcore
- doc/ bad error path fix ([#9](https://github.com/near/near-jsonrpc-client-rs/pull/9))
- update nearcore; fix validators method
- add default connector, update documentation
- ensure JsonRpcClient is reusable, yet completely consumable
- bump nearcore version
- properly deserialize info string from InternalError variants
- update to borsh 0.9
- *(methods)* add generic documentation on all rpc methods ([#8](https://github.com/near/near-jsonrpc-client-rs/pull/8))
- use traits, drop HTTP endpoint support, update docs ([#6](https://github.com/near/near-jsonrpc-client-rs/pull/6))
- add CI for auto-generating docs
- rename crate to near_jsonrpc_client ([#5](https://github.com/near/near-jsonrpc-client-rs/pull/5))
- update nearcore repo
- update EXPERIMENTAL_changes{,in_block} ([#4](https://github.com/near/near-jsonrpc-client-rs/pull/4))
- update nearcore repo
- update rpc method signature: [next_light_client_block]
- use the BlockReference from near-primitives
- add archival rpc urls
- update method signatures for all adversarial methods
- update rpc method signature: [EXPERIMENTAL_tx_status]
- update rpc method signature: [sandbox_patch_state]
- update rpc method signature: [EXPERIMENTAL_validators_ordered]
- update rpc method signature: [EXPERIMENTAL_receipt]
- update rpc method signature: [EXPERIMENTAL_protocol_config]
- update rpc method signature: [EXPERIMENTAL_genesis_config]
- update rpc method signature: [EXPERIMENTAL_check_tx]
- update rpc method signature: [EXPERIMENTAL_changes_in_block]
- update rpc method signature: [EXPERIMENTAL_changes]
- update rpc method signature: [EXPERIMENTAL_broadcast_tx_sync]
- properly serialize signed transaction to be sent over the wire
- update rpc method signature: [validators]
- update rpc method signature: [tx]
- update rpc+http method signature: [status]
- update rpc method signature: [query]
- update rpc method signature: [network_info]
- update rpc method signature: [next_light_client_block]
- update rpc method signature: [light_client_proof]
- add consts for near-hosted rpc endpoints
- update rpc method signature: [health]
- update rpc method signature: [gas_price]
- update rpc method signature: [chunk]
- update rpc method signature: [broadcast_tx_commit]
- update rpc method signature: [broadcast_tx_async]
- update rpc method signature: [block]
- impl next_light_client_block after Deserialize blocker has been cleared
- refactor method signatures, begin to get rid of the non-specific methodexecutionerror
- use manual Err+Display+Debug impls for generic structures
- reorder {Experimental,}RpcMethod-s alphabetically
- update doc code snippets, ensure all tests pass
- impl Error: fmt::{Debug,Display} for all Error-likes
- implement HTTP API method call support with strict error handling
- use RpcHealthResponse as return type for health()
- add comments, not docs to the RPC errors
- update description
- add initial documentation
- add status test via HTTP API
- use testnet addr on README
- impl Clone for both Near{Http,JsonRpc}Client
- be explicit for clarity: rpc -> jsonrpc
- :{new_client -> new}
- refactor providers, decouple rpc/http, split into modules
- appropriately rename crate to near-api-providers-rs
- impl adversarial features
- impl sandbox_patch_state
- impl EXPERIMENTAL_changes_in_block
- impl light_client_proof and network_info as non-experimental methods
- use null instead of an empty array for unparameterized rpc methods
- use strict types for error handling
- add method cases to namespace and compress Rpc*Request variants to single-value tuples
- revert nearcore deps to upstream repo
- deprecation message in lowercase
- bump nearcore version to latest branch revision
- lock actix_derive version
- use builder pattern for client creation
- properly rename crate, drop the `-rs`
- add deprecation note, do not use
- add do-not-use disclaimer to readme
- add missing hash param in tx method
- use SignedTransactionView for tx args instead of String
- remove query_by_path and block_by_id
- init near-api

## [0.5.1] - 2023-03-22

- Updated `borsh` to `0.10.2`. <https://github.com/near/near-jsonrpc-client-rs/pull/124>

## [0.5.0] - 2023-02-24

### Added

- `ApiKey::new` now accepts byte arrays and byte slices. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- `Authorization::bearer` method for token-authenticated requests. <https://github.com/near/near-jsonrpc-client-rs/pull/121>
- `ApiKey::as_bytes` returns a byte slice of the key without utf-8 validation. <https://github.com/near/near-jsonrpc-client-rs/pull/119>

### Changed

- Updated nearcore dependencies to `0.16.0`, which now requires a MSRV of `1.67.1`. <https://github.com/near/near-jsonrpc-client-rs/pull/122>
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
