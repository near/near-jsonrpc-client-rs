use super::*;

#[derive(Debug)]
pub struct RpcAdversarialSwitchToHeightRequest {
    pub height: u64,
}

impl RpcMethod for RpcAdversarialSwitchToHeightRequest {
    type Response = ();
    type Error = ();

    fn method_name(&self) -> &str {
        "adv_switch_to_height"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!([self.height]))
    }
}

impl private::Sealed for RpcAdversarialSwitchToHeightRequest {}
