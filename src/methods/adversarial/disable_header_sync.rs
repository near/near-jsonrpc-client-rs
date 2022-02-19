use super::*;

#[derive(Debug)]
pub struct RpcAdversarialDisableHeaderSyncRequest;

impl RpcMethod for RpcAdversarialDisableHeaderSyncRequest {
    type Response = ();
    type Error = ();

    fn method_name(&self) -> &str {
        "adv_disable_header_sync"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcAdversarialDisableHeaderSyncRequest {}
