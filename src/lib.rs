#![deprecated(note = "this crate is unstable and hence, unfit for use.")]

pub mod http;
pub mod rpc;

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
    pub fn new_client() -> NearClientBuilder {
        NearClientBuilder {
            client: reqwest::Client::new(),
        }
    }

    pub fn as_rpc(&self) -> rpc::NearRpcClient {
        rpc::NearRpcClient {
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
    use crate::{rpc::RpcMethod, NearClient};

    #[tokio::test]
    async fn it_works() {
        let rpc_client = NearClient::new_client()
            .connect("http://localhost:3030")
            .as_rpc();
        let status1 = rpc_client.status().await;
        let status2 = RpcMethod::Status
            .call_on::<near_primitives::views::StatusResponse>(&rpc_client)
            .await;

        println!("{:?}", status1.unwrap());
        println!("{:?}", status2.unwrap());
    }
}
