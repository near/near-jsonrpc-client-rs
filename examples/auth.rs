use near_jsonrpc_client::errors::{
    JsonRpcError::ServerError, JsonRpcServerError::ResponseStatusError,
    JsonRpcServerResponseStatusError::Unauthorized,
};
use near_jsonrpc_client::{auth, methods, JsonRpcClient};
use near_primitives::types::{BlockReference, Finality};

mod utils;

async fn unauthorized() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://near-mainnet.api.pagoda.co/rpc/v1/");

    let request = methods::block::RpcBlockRequest {
        block_reference: BlockReference::Finality(Finality::Final),
    };

    match client.call(request).await {
        Ok(_) => panic!("The unauthorized request succeeded unexpectedly."),
        Err(ServerError(ResponseStatusError(Unauthorized))) => {
            eprintln!("\x1b[33mThe unauthorized request failed as expected.\x1b[0m");
        }
        Err(error) => {
            eprintln!("\x1b[31mThe unauthorized request failed with an unexpected error.\x1b[0m");
            eprintln!("Error: {:#?}", error);
        }
    }

    Ok(())
}

async fn authorized(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://near-mainnet.api.pagoda.co/rpc/v1/")
        .header(auth::ApiKey::new(api_key)?);

    let request = methods::block::RpcBlockRequest {
        block_reference: BlockReference::Finality(Finality::Final),
    };

    match client.call(request).await {
        Ok(block) => println!("{:#?}", block),
        Err(error) => {
            eprintln!(
                "\x1b[31mThe authorized request failed unexpectedly, is the API key valid?\x1b[0m"
            );
            match error {
                ServerError(ResponseStatusError(Unauthorized)) => {
                    println!("Unauthorized: {}", error)
                }
                _ => println!("Unexpected error: {}", error),
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let input = utils::input("Enter an API Key: ")?;

    authorized(&input).await?;

    unauthorized().await?;

    Ok(())
}
