use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality};
use near_primitives::views::QueryRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::ViewCode {
            account_id: "nosedive.testnet".parse()?,
        },
    };

    let response = client.call(request).await?;

    if let QueryResponseKind::ViewCode(result) = response.kind {
        let path = "/tmp/nosedive.testnet.wasm";
        println!("⚙️  [nosedive.testnet]");
        println!("🏋        size: {} bytes", result.code.len());
        std::fs::write(path, result.code)?;
        println!("💾   saved to: {}", path);
    }

    Ok(())
}
