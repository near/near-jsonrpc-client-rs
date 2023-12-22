use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;

use serde::Deserialize;
use serde_json::{from_slice, json};

mod utils;

#[derive(Debug, Deserialize)]
pub struct AccountStatus {
    pub rating: f32,
    pub given: u64,
    pub received: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    let account_id = utils::input("Enter the account to view: ")?;

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "nosedive.testnet".parse()?,
            method_name: "status".to_string(),
            args: FunctionArgs::from(
                json!({
                    "account_id": account_id,
                })
                .to_string()
                .into_bytes(),
            ),
        },
    };

    let response = client.call(request).await?;

    if let QueryResponseKind::CallResult(result) = response.kind {
        println!("{:#?}", from_slice::<AccountStatus>(&result.result)?);
    }

    Ok(())
}
