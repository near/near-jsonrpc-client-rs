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

    let account_id: near_primitives::types::AccountId =
        utils::input("Enter the Account ID whose keys we're listing: ")?.parse()?;

    // `view_access_key_list` is paginated since nearcore 2.14. A request with neither
    // `limit` nor `after_key` is the legacy unpaginated form, which fails with
    // `TooManyAccessKeys` once the account holds more keys than the node's cap, so
    // pass a page size from the first request on. `last_key` is the cursor for the
    // next page and is `None` on the last one.
    let mut after_key = None;
    loop {
        let access_key_query_response = client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request: near_primitives::views::QueryRequest::ViewAccessKeyList {
                    account_id: account_id.clone(),
                    after_key,
                    limit: Some(50.try_into()?),
                },
            })
            .await?;

        let QueryResponseKind::AccessKeyList(response) = access_key_query_response.kind else {
            break;
        };
        for access_key in response.keys {
            println!("🗝 [{}]", access_key.public_key);
            println!("     \u{21b3}      nonce: {}", access_key.access_key.nonce);
            println!(
                "     \u{21b3} permission: {}",
                indent(20, format!("{:#?}", access_key.access_key.permission))
            );
        }

        match response.last_key {
            Some(last_key) => after_key = Some(last_key),
            None => break,
        }
    }

    Ok(())
}
