use near_jsonrpc_client::methods;
use near_primitives::types::{BlockReference, Finality};

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = utils::select_network()?;

    let request = methods::block::RpcBlockRequest {
        block_reference: BlockReference::Finality(Finality::Final),
    };

    let response = client.call(request).await?;

    println!("{:#?}", response);

    Ok(())
}
