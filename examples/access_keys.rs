use near_jsonrpc_client::methods;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::BlockReference;

mod utils;

fn indent(indentation: usize, s: String) -> String {
    let mut lines = s.split_inclusive("\n");
    let mut r = lines.next().unwrap().to_string();
    for l in lines {
        r.push_str(&" ".repeat(indentation - 3));
        r.push_str("\x1b[38;5;244m>\x1b[0m ");
        r.push_str(l);
    }
    r
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = utils::select_network()?;

    let account_id = utils::input("Enter the Account ID whose keys we're listing: ")?.parse()?;

    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKeyList { account_id },
        })
        .await?;

    if let QueryResponseKind::AccessKeyList(response) = access_key_query_response.kind {
        for access_key in response.keys {
            println!("🗝 [{}]", access_key.public_key);
            println!("     \u{21b3}      nonce: {}", access_key.access_key.nonce);
            println!(
                "     \u{21b3} permission: {}",
                indent(20, format!("{:#?}", access_key.access_key.permission))
            );
        }
    }

    Ok(())
}
