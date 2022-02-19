use super::*;

use serde::Deserialize;

#[derive(Debug)]
pub struct RpcAdversarialGetSavedBlocksRequest;

#[derive(Debug, Deserialize)]
pub struct RpcAdversarialGetSavedBlocksResponse(pub u64);

impl RpcHandlerResponse for RpcAdversarialGetSavedBlocksResponse {}

impl RpcMethod for RpcAdversarialGetSavedBlocksRequest {
    type Response = RpcAdversarialGetSavedBlocksResponse;
    type Error = ();

    fn method_name(&self) -> &str {
        "adv_get_saved_blocks"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcAdversarialGetSavedBlocksRequest {}
