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
//! # use near_api_providers::NearClient;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let near_client = NearClient::new().connect("https://rpc.testnet.near.org");
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

pub mod http;
pub mod jsonrpc;

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

impl NearClient {
    /// Construct a new NearClient for any server.
    ///
    /// If you intend to use the client more than once,
    /// it is advised to create a client once and **reuse** it.
    ///
    /// ## Example
    ///
    /// ```
    /// # use near_api_providers::NearClient;
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

    /// Create a dedicated client for querying the server via RPC API.
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
