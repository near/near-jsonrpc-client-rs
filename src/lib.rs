//! Generic, low-level interfaces for interacting with the NEAR Protocol via JSON_RPC / HTTP.
//!
//! It's recommended to use the higher-level `near-api` library instead. Rust version coming soon.
//!
//! ## Example
//!
//! Connect to the testnet RPC endpoint and request server status
//!
//! ```
//! # #![allow(deprecated)]
//! use near_jsonrpc_client::{methods, JsonRpcClient, NEAR_TESTNET_RPC_URL};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let testnet_client = JsonRpcClient::new().connect(NEAR_TESTNET_RPC_URL);
//!
//! let status_request = methods::status::RpcStatusRequest;
//! let server_status = testnet_client.call(&status_request).await?;
//!
//! println!("{:?}", server_status);
//! # Ok(())
//! # }
//! ```

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind};
use near_jsonrpc_primitives::message::{from_slice, Message};

pub mod errors;
pub mod methods;

use errors::*;

pub const NEAR_MAINNET_RPC_URL: &str = "https://rpc.mainnet.near.org";
pub const NEAR_TESTNET_RPC_URL: &str = "https://rpc.testnet.near.org";
pub const NEAR_MAINNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.mainnet.near.org";
pub const NEAR_TESTNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.testnet.near.org";

/// NEAR JSON RPC client connector.
#[derive(Clone)]
pub struct JsonRpcClientConnector {
    client: reqwest::Client,
}

impl JsonRpcClientConnector {
    pub fn connect(&self, server_addr: &str) -> JsonRpcClient {
        JsonRpcClient {
            server_addr: server_addr.to_string(),
            client: self.client.clone(),
        }
    }
}

/// A NEAR JSON RPC Client.
#[derive(Debug, Clone)]
pub struct JsonRpcClient {
    server_addr: String,
    client: reqwest::Client,
}

pub type JsonRpcMethodCallResult<T, E> = Result<T, JsonRpcError<E>>;

impl JsonRpcClient {
    /// Create a new client connector.
    ///
    /// If you intend to use the client more than once,
    /// it is advised to create a client once and **reuse** it.
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
    #[deprecated(note = "this crate is still under development")]
    pub fn new() -> JsonRpcClientConnector {
        JsonRpcClientConnector {
            client: reqwest::Client::new(),
        }
    }

    /// Method executor for the client.
    pub async fn call<M: methods::RpcMethod>(
        &self,
        method: &M,
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
            .client
            .post(&self.server_addr)
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
            let response_result = response.result.or_else(|err| {
                let err = match if err.error_struct.is_some() {
                    err
                } else {
                    loop {
                        if let RpcError { data: Some(err), .. } = err {
                            if let Ok(info) = serde_json::from_value::<String>(err) {
                                break RpcError::new_internal_error(None, info);
                            };
                        };
                        break RpcError::new_internal_error(None, format!("<no data>"));
                    }
                }
                .error_struct
                .unwrap()
                {
                    RpcErrorKind::HandlerError(handler_error) => {
                        JsonRpcError::ServerError(JsonRpcServerError::HandlerError(
                            serde_json::from_value(handler_error).map_err(|err| {
                                JsonRpcError::TransportError(RpcTransportError::RecvError(
                                    JsonRpcTransportRecvError::ResponseParseError(
                                        JsonRpcTransportHandlerResponseError::ErrorMessageParseError(
                                            err,
                                        ),
                                    ),
                                ))
                            })?,
                        ))
                    }
                    RpcErrorKind::RequestValidationError(err) => {
                        JsonRpcError::ServerError(JsonRpcServerError::RequestValidationError(err))
                    }
                    RpcErrorKind::InternalError(err) => {
                        JsonRpcError::ServerError(JsonRpcServerError::InternalError(err))
                    }
                };
                Err(err)
            })?;
            return M::parse_result(response_result).map_err(|err| {
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
        let client = JsonRpcClient::new().connect(RPC_SERVER_ADDR);
        let method = methods::status::RpcStatusRequest;
        let status = client.call(&method).await;

        println!("status of [{}]: {:?}", RPC_SERVER_ADDR, status.unwrap());
    }
}
