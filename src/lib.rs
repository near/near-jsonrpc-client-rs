#![deprecated(note = "this crate is unstable and hence, unfit for use.")]

//! Generic, low-level interfaces for interacting with the NEAR Protocol via JSON_RPC / HTTP.
//!
//! It's recommended to use the higher-level `near-api` library instead, rust crate coming soon.
//!
//! ## Example
//!
//! Connect and request status via JSON_RPC & HTTP API
//!
//! ```
//! # #![allow(deprecated)]
//! use near_jsonrpc_client::{NearClient, NEAR_TESTNET_RPC_URL};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let near_client = NearClient::new().connect(NEAR_TESTNET_RPC_URL);
//!
//! let jsonrpc_client = near_client.as_jsonrpc();
//! let http_client    = near_client.as_http()   ;
//!
//! let status_from_jsonrpc = jsonrpc_client.status().await?;
//! let status_from_http    = http_client   .status().await?;
//!
//! println!("{:?}", status_from_http);
//! # Ok(())
//! # }
//! ```

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind};
use near_jsonrpc_primitives::message::{from_slice, Message};

pub mod errors;
pub mod http;
pub mod jsonrpc;
pub mod methods;

use errors::*;

pub const NEAR_MAINNET_RPC_URL: &str = "https://rpc.mainnet.near.org";
pub const NEAR_TESTNET_RPC_URL: &str = "https://rpc.testnet.near.org";
pub const NEAR_MAINNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.mainnet.near.org";
pub const NEAR_TESTNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.testnet.near.org";

/// A generic RPC/HTTP NEAR Client builder.
///
/// Use this to create dedicated clients for each server.
#[derive(Clone)]
pub struct NearClientBuilder {
    client: reqwest::Client,
}

impl NearClientBuilder {
    /// Create a dedicated, generic client for connecting to the server.
    pub fn connect(&self, server_addr: &str) -> NearClient {
        NearClient {
            server_addr: server_addr.to_string(),
            client: self.client.clone(),
        }
    }
}

/// A generic RPC/HTTP NEAR Client.
///
/// Use this to construct more specific clients upon
/// which helper methods can be called. (See [NearClient::new])
#[derive(Debug, Clone)]
pub struct NearClient {
    server_addr: String,
    client: reqwest::Client,
}

pub type JsonRpcMethodCallResult<T, E> = Result<T, JsonRpcError<E>>;

impl NearClient {
    /// Construct a new NearClient for any server.
    ///
    /// If you intend to use the client more than once,
    /// it is advised to create a client once and **reuse** it.
    ///
    /// ## Example
    ///
    /// ```
    /// # use near_jsonrpc_client::NearClient;
    /// let client_builder = NearClient::new();
    ///
    /// let near_mainnet_client = client_builder.connect("https://rpc.mainnet.near.org");
    /// let near_testnet_client = client_builder.connect("https://rpc.testnet.near.org");
    /// ```
    pub fn new() -> NearClientBuilder {
        NearClientBuilder {
            client: reqwest::Client::new(),
        }
    }

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
            return serde_json::from_value(response_result).map_err(|err| {
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

    /// Create a dedicated client for querying the server via RPC API.
    #[deprecated(note = "deprecacted in favor of NearClient::call() the and RpcMethod trait")]
    pub fn as_jsonrpc(&self) -> jsonrpc::NearJsonRpcClient {
        jsonrpc::NearJsonRpcClient {
            near_client: self.clone(),
        }
    }

    /// Create a dedicated client for querying the server via HTTP API.
    pub fn as_http(&self) -> http::NearHttpClient {
        http::NearHttpClient {
            near_client: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{jsonrpc::JsonRpcMethod, NearClient};

    const RPC_SERVER_ADDR: &'static str = "http://localhost:3030";

    #[tokio::test]
    async fn check_jsonrpc_status() {
        let jsonrpc_client = NearClient::new().connect(RPC_SERVER_ADDR).as_jsonrpc();
        let status1 = jsonrpc_client.status().await;

        let status2 = JsonRpcMethod::Status
            .call_on::<near_primitives::views::StatusResponse, ()>(&jsonrpc_client)
            .await;

        println!("status via JSON_RPC method 1: {:?}", status1.unwrap());
        println!("status via JSON_RPC method 2: {:?}", status2.unwrap());
    }

    #[tokio::test]
    async fn check_http_status() {
        let http_client = NearClient::new().connect(RPC_SERVER_ADDR).as_http();

        let status = http_client.status().await;

        println!("status via HTTP: {:?}", status.unwrap());
    }
}
