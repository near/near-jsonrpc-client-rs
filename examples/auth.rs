use near_jsonrpc_client::errors::{
    JsonRpcError::ServerError, JsonRpcServerError::ResponseStatusError,
    JsonRpcServerResponseStatusError::Unauthorized,
};
use near_jsonrpc_client::{auth, methods, JsonRpcClient};
use near_primitives::types::{BlockReference, Finality};

async fn unauthorized() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://testnet.rpc.near.dev/");

    let response = client.call(methods::status::RpcStatusRequest).await;

    debug_assert!(
        matches!(
            response,
            Err(ServerError(ResponseStatusError(Unauthorized)))
        ),
        "got {:?}",
        response
    );

    Ok(())
}

async fn authorized() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://testnet.rpc.near.dev/")
        .auth(auth::ApiKey::new("399ba741-e939-4ffa-8c3c-306ec36fa8de"));

    let request = methods::block::RpcBlockRequest {
        block_reference: BlockReference::Finality(Finality::Final),
    };

    let response = client.call(request).await?;

    println!("{:#?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    unauthorized().await?;

    authorized().await?;

    Ok(())
}
