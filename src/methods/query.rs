use super::*;

pub use near_jsonrpc_primitives::types::query::{RpcQueryError, RpcQueryRequest, RpcQueryResponse};

impl RpcHandlerResponse for RpcQueryResponse {}

impl RpcHandlerError for RpcQueryError {}

impl private::Sealed for RpcQueryRequest {}

impl RpcMethod for RpcQueryRequest {
    type Response = RpcQueryResponse;
    type Error = RpcQueryError;

    fn method_name(&self) -> &str {
        "query"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(self))
    }

    fn parse_handler_response(
        response: serde_json::Value,
    ) -> Result<Result<Self::Response, Self::Error>, serde_json::Error> {
        match serde_json::from_value::<QueryResponse>(response)? {
            QueryResponse::HandlerResponse(r) => Ok(Ok(r)),
            QueryResponse::HandlerError(LegacyQueryError {
                error,
                block_height,
                block_hash,
            }) => {
                let mut err_parts = error.split(' ');
                let query_error = if let (
                    Some("access"),
                    Some("key"),
                    Some(pk),
                    Some("does"),
                    Some("not"),
                    Some("exist"),
                    Some("while"),
                    Some("viewing"),
                    None,
                ) = (
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                    err_parts.next(),
                ) {
                    let public_key = pk
                        .parse::<near_crypto::PublicKey>()
                        .map_err(serde::de::Error::custom)?;
                    RpcQueryError::UnknownAccessKey {
                        public_key,
                        block_height,
                        block_hash,
                    }
                } else {
                    RpcQueryError::ContractExecutionError {
                        vm_error: error,
                        block_height,
                        block_hash,
                    }
                };

                Ok(Err(query_error))
            }
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum QueryResponse {
    HandlerResponse(RpcQueryResponse),
    HandlerError(LegacyQueryError),
}

#[derive(serde::Deserialize)]
struct LegacyQueryError {
    error: String,
    block_height: near_primitives::types::BlockHeight,
    block_hash: near_primitives::hash::CryptoHash,
}

#[cfg(test)]
mod tests {
    use {super::*, crate::*};

    #[tokio::test]
    async fn test_unknown_access_key() -> Result<(), Box<dyn std::error::Error>> {
        let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");

        let request = RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::BlockId(
                near_primitives::types::BlockId::Height(63503911),
            ),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: "miraclx.testnet".parse()?,
                public_key: "ed25519:9KnjTjL6vVoM8heHvCcTgLZ67FwFkiLsNtknFAVsVvYY".parse()?,
            },
        };

        let response = client.call(request).await.unwrap_err();

        let err = response.handler_error()?;
        assert!(matches!(
            err,
            RpcQueryError::UnknownAccessKey {
                ref public_key,
                block_height: 63503911,
                ..
            } if public_key.to_string() == "ed25519:9KnjTjL6vVoM8heHvCcTgLZ67FwFkiLsNtknFAVsVvYY"
        ),);

        Ok(())
    }

    #[tokio::test]
    async fn test_contract_execution_error() -> Result<(), Box<dyn std::error::Error>> {
        let client = JsonRpcClient::connect("https://archival-rpc.testnet.near.org");

        let request = RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::BlockId(
                near_primitives::types::BlockId::Height(63503911),
            ),
            request: near_primitives::views::QueryRequest::CallFunction {
                #[allow(deprecated)]
                account_id: "miraclx.testnet".parse()?,
                method_name: "".to_string(),
                args: vec![].into(),
            },
        };

        let response = client.call(request).await.unwrap_err();

        let err = response.handler_error()?;
        assert!(
            matches!(
                err,
                RpcQueryError::ContractExecutionError {
                    ref vm_error,
                    block_height: 63503911,
                    ..
                } if vm_error.contains("FunctionCallError(MethodResolveError(MethodEmptyName))")
            ),
            "{:?}",
            err
        );

        Ok(())
    }
}
