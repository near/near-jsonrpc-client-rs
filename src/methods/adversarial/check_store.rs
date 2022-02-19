use super::*;

use serde::Deserialize;

#[derive(Debug)]
pub struct RpcAdversarialCheckStoreRequest;

#[derive(Debug, Deserialize)]
pub struct RpcAdversarialCheckStoreResponse(pub u64);

impl RpcHandlerResponse for RpcAdversarialCheckStoreResponse {}

impl RpcMethod for RpcAdversarialCheckStoreRequest {
    type Response = RpcAdversarialCheckStoreResponse;
    type Error = ();

    fn method_name(&self) -> &str {
        "adv_check_store"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcAdversarialCheckStoreRequest {}
