//! Lower-level API for interfacing with the NEAR Protocol via JSONRPC.
//!
//! ## Example
//!
//! Connect to the testnet RPC endpoint and request server status
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_TESTNET_RPC_URL};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let testnet_client = JsonRpcClient::connect(NEAR_TESTNET_RPC_URL);
//!
//! let status_request = methods::status::RpcStatusRequest;
//! let server_status = testnet_client.call(status_request).await?;
//!
//! println!("{:?}", server_status);
//! # Ok(())
//! # }
//! ```

use std::{fmt, sync::Arc};

use near_jsonrpc_primitives::errors::RpcErrorKind;
use near_jsonrpc_primitives::message::{from_slice, Message};

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
    /// let webClient = reqwest::Client::builder()
    ///     .proxy(reqwest::Proxy::all("https://192.168.1.1:4825")?)
    ///     .build()?;
    ///
    /// let testnet_client = JsonRpcClient::with(webClient).connect("https://rpc.testnet.near.org");
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
    ) -> JsonRpcMethodCallResult<M::Result, M::Error> {
        let (method_name, params) = (
            M::METHOD_NAME,
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
        let request = self
            .inner
            .client
            .post(&self.inner.server_addr)
            .header("Content-Type", "application/json")
            .body(request_payload);
        let response = request.send().await.map_err(|err| {
            JsonRpcError::TransportError(RpcTransportError::SendError(
                JsonRpcTransportSendError::PayloadSendError(err),
            ))
        })?;
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
            let response_result = match response.result {
                Ok(response_result) => response_result,
                Err(err) => {
                    let mut handler_parse_error = None;
                    match err.error_struct {
                        Some(RpcErrorKind::HandlerError(handler_error)) => {
                            match serde_json::from_value(handler_error) {
                                Ok(handler_error) => {
                                    return Err(JsonRpcError::ServerError(
                                        JsonRpcServerError::HandlerError(handler_error),
                                    ))
                                }
                                Err(err) => {
                                    handler_parse_error.replace(err);
                                }
                            }
                        }
                        Some(RpcErrorKind::RequestValidationError(err)) => {
                            return Err(JsonRpcError::ServerError(
                                JsonRpcServerError::RequestValidationError(err),
                            ));
                        }
                        Some(RpcErrorKind::InternalError(err)) => {
                            return Err(JsonRpcError::ServerError(
                                JsonRpcServerError::InternalError {
                                    info: err["info"]["error_message"]
                                        .as_str()
                                        .unwrap_or("<no data>")
                                        .to_string(),
                                },
                            ))
                        }
                        None => {}
                    }
                    if let Some(raw_err_data) = err.data {
                        match methods::RpcHandlerError::parse_raw_error(raw_err_data) {
                            Some(Ok(handler_error)) => {
                                return Err(JsonRpcError::ServerError(
                                    JsonRpcServerError::HandlerError(handler_error),
                                ))
                            }
                            Some(Err(err)) => {
                                handler_parse_error.replace(err);
                            }
                            None => {}
                        }
                    }
                    if let Some(err) = handler_parse_error {
                        return Err(JsonRpcError::TransportError(RpcTransportError::RecvError(
                            JsonRpcTransportRecvError::ResponseParseError(
                                JsonRpcTransportHandlerResponseError::ErrorMessageParseError(err),
                            ),
                        )));
                    }
                    return Err(JsonRpcError::ServerError(
                        JsonRpcServerError::NonContextualError {
                            code: err.code,
                            message: err.message,
                        },
                    ));
                }
            };
            return methods::RpcHandlerResult::parse_result(response_result).map_err(|err| {
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

    const RPC_SERVER_ADDR: &'static str = "http://localhost:3030";

    #[tokio::test]
    async fn check_jsonrpc_status() {
        let client = JsonRpcClient::connect(RPC_SERVER_ADDR);
        let method = methods::status::RpcStatusRequest;
        let status = client.call(method).await;

        println!("status of [{}]: {:?}", RPC_SERVER_ADDR, status.unwrap());
    }
}
