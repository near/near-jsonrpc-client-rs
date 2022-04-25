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
//!    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//!    let request = methods::status::RpcStatusRequest; // no params
//!
//!    // call a method on the server via the connected client
//!    let server_status = client.call(request).await?;
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
//!    let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//!
//!    let tx_status_request = methods::tx::RpcTransactionStatusRequest {
//!        transaction_info: TransactionInfo::TransactionId {
//!            hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse()?,
//!            account_id: "miraclx.near".parse()?,
//!        },
//!    };
//!
//!    let tx_status = client.call(tx_status_request).await?;
//!
//!    println!("{:?}", tx_status);
//!    # Ok(())
//!    # }
//!    ```
use std::{fmt, sync::Arc};

use lazy_static::lazy_static;

use near_jsonrpc_primitives::message::{from_slice, Message};

#[cfg(feature = "auth")]
pub mod auth;
pub mod errors;
pub mod header;
pub mod methods;

use errors::*;

pub const NEAR_MAINNET_RPC_URL: &str = "https://rpc.mainnet.near.org";
pub const NEAR_TESTNET_RPC_URL: &str = "https://rpc.testnet.near.org";
pub const NEAR_MAINNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.mainnet.near.org";
pub const NEAR_TESTNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.testnet.near.org";

lazy_static! {
    static ref DEFAULT_CONNECTOR: JsonRpcClientConnector = JsonRpcClient::new_client();
}

/// NEAR JSON RPC client connector.
#[derive(Clone)]
pub struct JsonRpcClientConnector {
    client: reqwest::Client,
}

impl JsonRpcClientConnector {
    /// Return a JsonRpcClient that connects to the specified server.
    pub fn connect<U: AsUrl>(&self, server_addr: U) -> JsonRpcClient {
        JsonRpcClient {
            inner: Arc::new(JsonRpcInnerClient {
                server_addr: server_addr.to_string(),
                client: self.client.clone(),
            }),
            headers: reqwest::header::HeaderMap::new(),
        }
    }
}

struct JsonRpcInnerClient {
    server_addr: String,
    client: reqwest::Client,
}

#[derive(Clone)]
/// A NEAR JSON RPC Client.
///
/// This is the main struct that you will use to interact with the NEAR JSON RPC API.
///
/// ## Example
///
///```
/// use near_jsonrpc_client::{methods, JsonRpcClient};
/// use near_primitives::types::{BlockReference, Finality};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
///
/// let request = methods::block::RpcBlockRequest {
///     block_reference: BlockReference::Finality(Finality::Final),
/// };
///
/// let server_status = client.call(request).await?;
///
/// println!("{:?}", server_status);
/// # Ok(())
/// # }
/// ```
pub struct JsonRpcClient {
    inner: Arc<JsonRpcInnerClient>,
    headers: reqwest::header::HeaderMap,
}

pub type MethodCallResult<T, E> = Result<T, JsonRpcError<E>>;

impl JsonRpcClient {
    /// Connect to a JSON RPC server using the default connector.
    ///
    /// It's virtually the same as calling `new_client().connect(server_addr)`.
    /// Only, this method optimally reuses the same connector across invocations.
    ///
    /// ## Example
    ///
    /// ```
    /// use near_jsonrpc_client::JsonRpcClient;
    ///
    /// let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
    /// ```
    pub fn connect<U: AsUrl>(server_addr: U) -> JsonRpcClient {
        DEFAULT_CONNECTOR.connect(server_addr)
    }

    /// Get the server address the client connects to.
    ///
    /// It basically returns the server address passed to `connect()`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use near_jsonrpc_client::JsonRpcClient;
    /// let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
    ///
    /// assert_eq!(client.server_addr(), "https://rpc.testnet.near.org");
    /// ```
    pub fn server_addr(&self) -> &str {
        &self.inner.server_addr
    }

    /// RPC method executor for the client.
    ///
    /// ## Example
    ///
    /// ```
    /// use near_jsonrpc_client::{methods, JsonRpcClient};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
    ///
    /// let request = methods::status::RpcStatusRequest;
    /// let response = client.call(request).await?;
    ///
    /// assert!(matches!(
    ///     response,
    ///     methods::status::RpcStatusResponse { .. }
    /// ));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call<M>(&self, method: M) -> MethodCallResult<M::Response, M::Error>
    where
        M: methods::RpcMethod,
    {
        let request_payload = methods::to_json(&method).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSerializeError(err),
            ))
        })?;

        let request_payload = serde_json::to_vec(&request_payload).map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSerializeError(err.into()),
            ))
        })?;

        let request = self
            .inner
            .client
            .post(&self.inner.server_addr)
            .headers(self.headers.clone())
            .body(request_payload);

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
                            JsonRpcServerResponseStatusError::TooManyRequests
                        }
                        unexpected => {
                            JsonRpcServerResponseStatusError::Unexpected { status: unexpected }
                        }
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
            return M::parse_handler_response(response.result?)
                .map_err(|err| {
                    JsonRpcError::TransportError(RpcTransportError::RecvError(
                        JsonRpcTransportRecvError::ResponseParseError(
                            JsonRpcTransportHandlerResponseError::ResultParseError(err),
                        ),
                    ))
                })?
                .map_err(|err| JsonRpcError::ServerError(JsonRpcServerError::HandlerError(err)));
        }
        Err(JsonRpcError::TransportError(RpcTransportError::RecvError(
            JsonRpcTransportRecvError::UnexpectedServerResponse(response_message),
        )))
    }

    /// Add a header to this request.
    ///
    /// Depending on the header specified, this method either returns back
    /// the client, or a result containing the client.
    ///
    /// ### Example
    ///
    /// ```
    /// use near_jsonrpc_client::JsonRpcClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
    /// let client = client.header(("user-agent", "someclient/0.1.0"))?; // <- returns a result
    ///
    /// # #[cfg(feature = "auth")]
    /// use near_jsonrpc_client::auth;
    ///
    /// # #[cfg(feature = "auth")]
    /// let client = client.header(
    ///     auth::ApiKey::new("cadc4c83-5566-4c94-aa36-773605150f44")?, // <- error handling here
    /// ); // <- returns the client
    /// # Ok(())
    /// # }
    /// ```
    pub fn header<H, D>(self, entry: H) -> D::Output
    where
        H: header::HeaderEntry<D>,
        D: header::HeaderEntryDiscriminant<H>,
    {
        D::apply(self, entry)
    }

    /// Get a shared reference to the headers.
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    /// Get an exclusive reference to the headers.
    pub fn headers_mut(&mut self) -> &mut reqwest::header::HeaderMap {
        &mut self.headers
    }

    /// Manually create a new client connector.
    ///
    /// It's recommended to use the [`connect`](JsonRpcClient::connect) method instead as that method optimally
    /// reuses the default connector across invocations.
    ///
    /// However, if for some reason you still need to manually create a new connector, you can do so.
    /// Just remember to properly **reuse** it as much as possible.
    ///
    /// ## Example
    ///
    /// ```
    /// # use near_jsonrpc_client::JsonRpcClient;
    /// let client_connector = JsonRpcClient::new_client();
    ///
    /// let mainnet_client = client_connector.connect("https://rpc.mainnet.near.org");
    /// let testnet_client = client_connector.connect("https://rpc.testnet.near.org");
    /// ```
    pub fn new_client() -> JsonRpcClientConnector {
        let mut headers = reqwest::header::HeaderMap::with_capacity(2);
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        JsonRpcClientConnector {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
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
}

impl fmt::Debug for JsonRpcClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("JsonRpcClient");
        builder.field("server_addr", &self.inner.server_addr);
        builder.field("headers", &self.headers);
        builder.field("client", &self.inner.client);
        builder.finish()
    }
}

mod private {
    pub trait Sealed: ToString {}
}

pub trait AsUrl: private::Sealed {}

impl private::Sealed for String {}

impl AsUrl for String {}

impl private::Sealed for &String {}

impl AsUrl for &String {}

impl private::Sealed for &str {}

impl AsUrl for &str {}

impl private::Sealed for reqwest::Url {}

impl AsUrl for reqwest::Url {}

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
