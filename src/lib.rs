//! Lower-level API for interfacing with the NEAR Protocol via JSONRPC.
//!
//! ## Layout
//!
//! Each one the valid *public* JSON RPC methods are pre-defined in specialized modules within the `methods` module.
//!
//! Inside every method module (e.g [`methods::query`]) there's;
//!   - a `Request` type (e.g [`methods::query::RpcQueryRequest`])
//!   - a `Response` type (e.g [`methods::query::RpcQueryResponse`])
//!   - and an `Error` type (e.g [`methods::query::RpcQueryError`])
//!
//! Calling a constructed request on a client returns with the result and error types for that method.
//!
//! ## Examples
//!
//! 1. Request server status from testnet RPC
//!
//!    ```
//!    # #![allow(deprecated)]
//!    use near_jsonrpc_client::{methods, JsonRpcClient};
//!
//!    # #[tokio::main]
//!    # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let testnet_client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//!    let status_request = methods::status::RpcStatusRequest; // no params
//!
//!    // call a method on the server via the connected client
//!    let server_status = testnet_client.call(status_request).await?;
//!
//!    println!("{:?}", server_status);
//!    # Ok(())
//!    # }
//!    ```
//!
//! 2. Query transaction status from mainnet RPC
//!
//!    ```
//!    use near_jsonrpc_client::{methods, JsonRpcClient};
//!    use near_jsonrpc_primitives::types::transactions::TransactionInfo;
//!
//!    # #[tokio::main]
//!    # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mainnet_client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//!    let tx_status_request = methods::tx::RpcTransactionStatusRequest {
//!        transaction_info: TransactionInfo::TransactionId {
//!            hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse()?,
//!            account_id: "miraclx.near".parse()?,
//!        },
//!    };
//!
//!    let tx_status = mainnet_client.call(tx_status_request).await?;
//!
//!    println!("{:?}", tx_status);
//!    # Ok(())
//!    # }
//!    ```
//!
//! 3. For all intents and purposes, the predefined structures in `methods` should suffice, if you find that they
//!    don't or you crave extra flexibility, well, you can opt in to use the generic constructor `methods::any()` with the `any` feature flag.
//!
//!    In this example, we retrieve only the parts from the genesis config response that we care about.
//!
//!    ```toml
//!    # in Cargo.toml
//!    near-jsonrpc-client = { ..., features = ["any"] }
//!    ```
//!
//!    ```
//!    use serde::Deserialize;
//!    use serde_json::json;
//!
//!    # use near_jsonrpc_client::errors::JsonRpcError;
//!    use near_jsonrpc_client::{methods, JsonRpcClient};
//!    use near_primitives::serialize::u128_dec_format;
//!    use near_primitives::types::*;
//!
//!    #[derive(Debug, Deserialize)]
//!    struct PartialGenesisConfig {
//!        protocol_version: ProtocolVersion,
//!        chain_id: String,
//!        genesis_height: BlockHeight,
//!        epoch_length: BlockHeightDelta,
//!        #[serde(with = "u128_dec_format")]
//!        min_gas_price: Balance,
//!        #[serde(with = "u128_dec_format")]
//!        max_gas_price: Balance,
//!        #[serde(with = "u128_dec_format")]
//!        total_supply: Balance,
//!        validators: Vec<AccountInfo>,
//!    }
//!
//!    impl methods::RpcHandlerResponse for PartialGenesisConfig {}
//!
//!    # #[tokio::main]
//!    # async fn main() -> Result<(), JsonRpcError<()>> {
//!    let mainnet_client = JsonRpcClient::connect("https://rpc.mainnet.near.org");
//!
//!    let genesis_config_request = methods::any::<Result<PartialGenesisConfig, ()>>(
//!        "EXPERIMENTAL_genesis_config",
//!        json!(null),
//!    );
//!
//!    let partial_genesis = mainnet_client.call(genesis_config_request).await?;
//!
//!    println!("{:#?}", partial_genesis);
//!    # Ok(())
//!    # }
//!    ```

use std::{convert::TryFrom, fmt, sync::Arc};

use near_jsonrpc_primitives::message::{from_slice, Message};
use near_primitives::serialize::to_base64;

use lazy_static::lazy_static;

pub mod errors;
pub mod methods;

use errors::*;

pub const NEAR_MAINNET_RPC_URL: &str = "https://rpc.mainnet.near.org";
pub const NEAR_TESTNET_RPC_URL: &str = "https://rpc.testnet.near.org";
pub const NEAR_MAINNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.mainnet.near.org";
pub const NEAR_TESTNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.testnet.near.org";

lazy_static! {
    static ref DEFAULT_CONNECTOR: JsonRpcClientConnector = JsonRpcClient::new();
}

#[derive(Eq, Clone, Debug, PartialEq)]
pub enum ClientCredentials {
    NoAuth,
    Basic(String),
}

/// NEAR JSON RPC client connector.
#[derive(Clone)]
pub struct JsonRpcClientConnector {
    client: reqwest::Client,
}

impl JsonRpcClientConnector {
    pub fn connect(&self, server_addr: &str) -> JsonRpcClient {
        JsonRpcClient {
            inner: Arc::new(JsonRpcInnerClient {
                server_addr: server_addr.to_string(),
                client: self.client.clone(),
            }),
            creds: ClientCredentials::NoAuth,
        }
    }
}

struct JsonRpcInnerClient {
    server_addr: String,
    client: reqwest::Client,
}

/// A NEAR JSON RPC Client.
#[derive(Clone)]
pub struct JsonRpcClient {
    inner: Arc<JsonRpcInnerClient>,
    creds: ClientCredentials,
}

impl fmt::Debug for JsonRpcClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("JsonRpcClient");
        builder.field("server_addr", &self.inner.server_addr);
        builder.field("client", &self.inner.client);
        builder.finish()
    }
}

pub type JsonRpcMethodCallResult<T, E> = Result<T, JsonRpcError<E>>;

impl JsonRpcClient {
    /// Connect to a JSON RPC server using the default connector.
    ///
    /// It's virtually the same as calling `new()` and then `connect(server_addr)`.
    /// Only, this method optimally reuses the same connector across invocations.
    ///
    /// ## Example
    ///
    /// ```
    /// use near_jsonrpc_client::{methods, JsonRpcClient};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mainnet_client = JsonRpcClient::connect("https://rpc.testnet.near.org");
    ///
    /// let status_request = methods::status::RpcStatusRequest;
    /// let server_status = mainnet_client.call(status_request).await?;
    ///
    /// println!("{:?}", server_status);
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect(server_addr: &str) -> JsonRpcClient {
        DEFAULT_CONNECTOR.connect(server_addr)
    }

    pub fn auth(mut self, creds: ClientCredentials) -> JsonRpcClient {
        self.creds = creds;
        self
    }

    pub fn server_addr(&self) -> &str {
        &self.inner.server_addr
    }

    /// Manually create a new client connector.
    ///
    /// It's recommended to use the `connect` method instead as that method optimally
    /// reuses the default connector across invocations.
    ///
    /// However, if for some reason you still need to manually create a new connector, you can do so.
    /// Just remember to properly **reuse** it as much as possible.
    ///
    /// ## Example
    ///
    /// ```
    /// # use near_jsonrpc_client::JsonRpcClient;
    /// let client_connector = JsonRpcClient::new();
    ///
    /// let mainnet_client = client_connector.connect("https://rpc.mainnet.near.org");
    /// let testnet_client = client_connector.connect("https://rpc.testnet.near.org");
    /// ```
    pub fn new() -> JsonRpcClientConnector {
        JsonRpcClientConnector {
            client: reqwest::Client::new(),
        }
    }

    /// Create a new client constructor using a custom web client.
    ///
    /// This is useful if you want to customize the `reqwest::Client` instance used by the JsonRpcClient.
    ///
    /// ## Example
    ///
    /// ```
    /// use near_jsonrpc_client::JsonRpcClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let web_client = reqwest::Client::builder()
    ///     .proxy(reqwest::Proxy::all("https://192.168.1.1:4825")?)
    ///     .build()?;
    ///
    /// let testnet_client = JsonRpcClient::with(web_client).connect("https://rpc.testnet.near.org");
    /// # Ok(())
    /// # }
    /// ```
    pub fn with(client: reqwest::Client) -> JsonRpcClientConnector {
        JsonRpcClientConnector { client }
    }

    /// Method executor for the client.
    pub async fn call<M: methods::RpcMethod>(
        self,
        method: M,
    ) -> JsonRpcMethodCallResult<M::Response, M::Error> {
        let (method_name, params) = (
            method.method_name(),
            method.params().map_err(|err| {
                JsonRpcError::TransportError(RpcTransportError::SendError(
                    JsonRpcTransportSendError::PayloadSerializeError(err),
                ))
            })?,
        );
        let request_payload = Message::request(method_name.to_string(), Some(params));
        let request_payload = serde_json::to_vec(&request_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSerializeError(err.into()),
            ))
        })?;
        let mut request = self
            .inner
            .client
            .post(&self.inner.server_addr)
            .header("Content-Type", "application/json")
            .body(request_payload);
        match self.creds {
            ClientCredentials::NoAuth => {}
            ClientCredentials::Basic(basic_token) => {
                request =
                    request.header("Authorization", format!("Basic {}", to_base64(basic_token)))
            }
        }
        let response = request.send().await.map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSendError(err),
            ))
        })?;
        match response.status() {
            reqwest::StatusCode::OK => {}
            non_ok_status => {
                return Err(JsonRpcError::ServerError(
                    JsonRpcServerError::ResponseStatusError(match non_ok_status {
                        reqwest::StatusCode::UNAUTHORIZED => {
                            JsonRpcServerResponseStatusError::Unauthorized
                        }
                        reqwest::StatusCode::TOO_MANY_REQUESTS => {
                            JsonRpcServerResponseStatusError::TooManyRequests {
                                retry_after: response
                                    .headers()
                                    .get("retry-after")
                                    .and_then(|directive| {
                                        retry_after::RetryAfter::try_from(directive).ok()
                                    })
                                    .map(Into::into),
                            }
                        }
                        unexpected => JsonRpcServerResponseStatusError::Unexpected(unexpected),
                    }),
                ));
            }
        }
        let response_payload = response.bytes().await.map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::RecvError(
                JsonRpcTransportRecvError::PayloadRecvError(err),
            ))
        })?;
        let response_message = from_slice(&response_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::RecvError(
                JsonRpcTransportRecvError::PayloadParseError(err),
            ))
        })?;

        if let Message::Response(response) = response_message {
            return methods::RpcHandlerResponse::parse(response.result?).map_err(|err| {
                JsonRpcError::TransportError(RpcTransportError::RecvError(
                    JsonRpcTransportRecvError::ResponseParseError(
                        JsonRpcTransportHandlerResponseError::ResultParseError(err),
                    ),
                ))
            });
        }
        Err(JsonRpcError::TransportError(RpcTransportError::RecvError(
            JsonRpcTransportRecvError::UnexpectedServerResponse(response_message),
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::{methods, JsonRpcClient};

    const RPC_SERVER_ADDR: &'static str = "https://archival-rpc.mainnet.near.org";

    #[tokio::test]
    async fn chk_status_testnet() {
        let client = JsonRpcClient::connect(RPC_SERVER_ADDR);

        let status = client.call(methods::status::RpcStatusRequest).await;

        assert!(
            matches!(status, Ok(methods::status::RpcStatusResponse { .. })),
            "expected an Ok(RpcStatusResponse), found [{:?}]",
            status
        );
    }

    #[tokio::test]
    #[cfg(feature = "any")]
    async fn any_typed_ok() -> Result<(), Box<dyn std::error::Error>> {
        let client = JsonRpcClient::connect(RPC_SERVER_ADDR);

        let tx_status = client
            .call(methods::any::<methods::tx::RpcTransactionStatusRequest>(
                "tx",
                serde_json::json!([
                    "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U",
                    "miraclx.near",
                ]),
            ))
            .await;

        assert!(
            matches!(
                tx_status,
                Ok(methods::tx::RpcTransactionStatusResponse { ref transaction, .. })
                if transaction.signer_id.as_ref() == "miraclx.near"
                && transaction.hash == "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse()?
            ),
            "expected an Ok(RpcTransactionStatusResponse) with matching signer_id + hash, found [{:?}]",
            tx_status
        );

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "any")]
    async fn any_typed_err() -> Result<(), Box<dyn std::error::Error>> {
        let client = JsonRpcClient::connect(RPC_SERVER_ADDR);

        let tx_error = client
            .call(methods::any::<methods::tx::RpcTransactionStatusRequest>(
                "tx",
                serde_json::json!([
                    "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8D",
                    "youser.near",
                ]),
            ))
            .await
            .expect_err("request must not succeed")
            .handler_error();

        assert!(
            matches!(
                tx_error,
                Ok(methods::tx::RpcTransactionError::UnknownTransaction {
                    requested_transaction_hash
                })
                if requested_transaction_hash == "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8D".parse()?
            ),
            "expected an Ok(RpcTransactionError::UnknownTransaction) with matching hash, found [{:?}]",
            tx_error
        );

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "any")]
    async fn any_untyped_ok() {
        let client = JsonRpcClient::connect(RPC_SERVER_ADDR);

        let status = client
            .call(
                methods::any::<Result<serde_json::Value, serde_json::Value>>(
                    "tx",
                    serde_json::json!([
                        "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U",
                        "miraclx.near",
                    ]),
                ),
            )
            .await
            .expect("request must not fail");

        assert_eq!(
            status["transaction"]["signer_id"], "miraclx.near",
            "expected a tx_status with matching signer_id, [{:#}]",
            status
        );
        assert_eq!(
            status["transaction"]["hash"], "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U",
            "expected a tx_status with matching hash, [{:#}]",
            status
        );
    }

    #[tokio::test]
    #[cfg(feature = "any")]
    async fn any_untyped_err() {
        let client = JsonRpcClient::connect(RPC_SERVER_ADDR);

        let tx_error = client
            .call(
                methods::any::<Result<serde_json::Value, serde_json::Value>>(
                    "tx",
                    serde_json::json!([
                        "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8D",
                        "youser.near",
                    ]),
                ),
            )
            .await
            .expect_err("request must not succeed")
            .handler_error()
            .expect("expected a handler error from query request");

        assert_eq!(
            tx_error["info"]["requested_transaction_hash"],
            "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8D",
            "expected an error with matching hash, [{:#}]",
            tx_error
        );
        assert_eq!(
            tx_error["name"], "UNKNOWN_TRANSACTION",
            "expected an UnknownTransaction, [{:#}]",
            tx_error
        );
    }
}
