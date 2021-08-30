#![deprecated(note = "this crate is unstable and hence, unfit for use.")]

pub mod http;
pub mod jsonrpc;

#[derive(Clone)]
pub struct NearClientBuilder {
    client: reqwest::Client,
}

impl NearClientBuilder {
    pub fn connect(&self, server_addr: &str) -> NearClient {
        NearClient {
            server_addr: server_addr.to_string(),
            client: self.client.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NearClient {
    server_addr: String,
    client: reqwest::Client,
}

impl NearClient {
    pub fn new() -> NearClientBuilder {
        NearClientBuilder {
            client: reqwest::Client::new(),
        }
    }

    pub fn as_jsonrpc(&self) -> jsonrpc::NearJsonRpcClient {
        jsonrpc::NearJsonRpcClient {
            near_client: self.clone(),
        }
    }

    pub fn as_http(&self) -> http::NearHttpClient {
        http::NearHttpClient {
            near_client: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{jsonrpc::JsonRpcMethod, NearClient};

    #[tokio::test]
    async fn check_jsonrpc_status() {
        let jsonrpc_client = NearClient::new()
            .connect("http://localhost:3030")
            .as_jsonrpc();
        let status1 = jsonrpc_client.status().await;

        let status2 = JsonRpcMethod::Status
            .call_on::<near_primitives::views::StatusResponse>(&jsonrpc_client)
            .await;

        println!("{:?}", status1.unwrap());
        println!("{:?}", status2.unwrap());
    }
}
