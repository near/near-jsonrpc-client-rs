use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::types::{BlockReference, Finality};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    let request = methods::block::RpcBlockRequest {
        block_reference: BlockReference::Finality(Finality::Final),
    };

    let response = client.call(request).await?;

    println!("{:?}", response);

    Ok(())
}
