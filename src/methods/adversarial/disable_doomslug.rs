use super::*;

#[derive(Debug)]
pub struct RpcAdversarialDisableDoomslugRequest;

impl RpcMethod for RpcAdversarialDisableDoomslugRequest {
    type Response = ();
    type Error = ();

    fn method_name(&self) -> &str {
        "adv_disable_doomslug"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl private::Sealed for RpcAdversarialDisableDoomslugRequest {}
