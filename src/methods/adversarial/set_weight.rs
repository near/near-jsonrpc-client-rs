use super::*;

#[derive(Debug)]
pub struct RpcAdversarialSetWeightRequest {
    pub height: u64,
}

impl RpcMethod for RpcAdversarialSetWeightRequest {
    type Response = ();
    type Error = ();

    fn method_name(&self) -> &str {
        "adv_set_weight"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self.height))
    }
}

impl private::Sealed for RpcAdversarialSetWeightRequest {}
