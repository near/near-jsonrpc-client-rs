use super::*;

#[derive(Debug)]
pub struct RpcAdversarialProduceBlocksRequest {
    pub num_blocks: u64,
    pub only_valid: bool,
}

impl RpcMethod for RpcAdversarialProduceBlocksRequest {
    type Response = ();
    type Error = ();

    fn method_name(&self) -> &str {
        "adv_produce_blocks"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!([self.num_blocks, self.only_valid]))
    }
}

impl private::Sealed for RpcAdversarialProduceBlocksRequest {}
