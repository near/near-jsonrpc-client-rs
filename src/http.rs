use super::NearClient;

pub struct NearHttpClient {
    pub(crate) near_client: NearClient,
}

impl NearHttpClient {
    pub async fn status() -> Result<
        near_jsonrpc_primitives::types::status::RpcStatusResponse,
        near_jsonrpc_primitives::types::status::RpcStatusError,
    > {
        todo!()
    }

    pub async fn health() -> Result<
        near_jsonrpc_primitives::types::status::RpcHealthResponse,
        near_jsonrpc_primitives::types::status::RpcStatusError,
    > {
        todo!()
    }

    pub async fn network_info() -> Result<
        near_jsonrpc_primitives::types::network_info::RpcNetworkInfoResponse,
        near_jsonrpc_primitives::types::network_info::RpcNetworkInfoError,
    > {
        todo!()
    }

    #[cfg(feature = "metrics")]
    pub async fn metrics() -> Result<Vec<prometheus::proto::MetricFamily>, ()> {
        todo!()
    }
}
