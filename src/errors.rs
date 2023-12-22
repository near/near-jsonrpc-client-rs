//! Error types.
use std::io;

use thiserror::Error;

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind, RpcRequestValidationErrorKind};
use near_jsonrpc_primitives::message::{self, Message};

/// Potential errors returned while sending a request to the RPC server.
#[derive(Debug, Error)]
pub enum JsonRpcTransportSendError {
    /// Client is unable to serialize the request payload before sending it to the server.
    #[error("error while serializing payload: [{0}]")]
    PayloadSerializeError(io::Error),
    /// Client is unable to send the request to the server.
    #[error("error while sending payload: [{0}]")]
    PayloadSendError(reqwest::Error),
}

/// Potential errors returned when the client has an issue parsing the response of a method call.
#[derive(Debug, Error)]
pub enum JsonRpcTransportHandlerResponseError {
    /// Client fails to deserialize the result of a method call.
    #[error("error while parsing method call result: [{0}]")]
    ResultParseError(serde_json::Error),
    /// Client fails to deserialize the error message returned from a method call.
    #[error("error while parsing method call error message: [{0}]")]
    ErrorMessageParseError(serde_json::Error),
}

/// Potential errors returned while receiving responses from an RPC server.
#[derive(Debug, Error)]
pub enum JsonRpcTransportRecvError {
    /// Client receives a JSON RPC message body that isn't structured as a response.
    #[error("unexpected server response: [{0:?}]")]
    UnexpectedServerResponse(Message),
    /// Client is unable to read the response from the RPC server.
    #[error("error while reading response: [{0}]")]
    PayloadRecvError(reqwest::Error),
    /// The base response structure is malformed e.g. meta properties like RPC version are missing.
    #[error("error while parsing server response: [{0:?}]")]
    PayloadParseError(message::Broken),
    /// Potential errors returned when the client has an issue parsing the response of a method call.
    #[error(transparent)]
    ResponseParseError(JsonRpcTransportHandlerResponseError),
}

/// Potential errors returned while sending requests to or receiving responses from the RPC server.
#[derive(Debug, Error)]
pub enum RpcTransportError {
    /// Potential errors returned while sending a request to the RPC server.
    #[error(transparent)]
    SendError(JsonRpcTransportSendError),
    /// Potential errors returned while receiving a response from an RPC server.
    #[error(transparent)]
    RecvError(JsonRpcTransportRecvError),
}

/// Unexpected status codes returned by the RPC server.
#[derive(Debug, Error)]
pub enum JsonRpcServerResponseStatusError {
    /// The RPC client is unauthorized.
    #[error("this client is unauthorized")]
    Unauthorized,
    /// The RPC client exceeds the rate limit by sending too many requests.
    #[error("this client has exceeded the rate limit")]
    TooManyRequests,
    /// The RPC server returned a non-200 status code.
    #[error("the server returned a non-OK (200) status code: [{status}]")]
    Unexpected { status: reqwest::StatusCode },
}

/// Potential errors returned by the RPC server.
#[derive(Debug, Error)]
pub enum JsonRpcServerError<E> {
    /// An invalid RPC method is called or the RPC methdo is unable to parse the provided arguments.
    #[error("request validation error: [{0:?}]")]
    RequestValidationError(RpcRequestValidationErrorKind),
    /// RPC method call error.
    #[error("handler error: [{0}]")]
    HandlerError(E),
    /// The RPC server returned an internal server error.
    #[error("internal error: [{info:?}]")]
    InternalError { info: Option<String> },
    /// The RPC server returned a response without context i.e. a response the client doesn't expect.
    #[error("error response lacks context: {0}")]
    NonContextualError(RpcError),
    /// Unexpected status codes returned by the RPC server.
    #[error(transparent)]
    ResponseStatusError(JsonRpcServerResponseStatusError),
}

/// Potential errors returned by the RPC client.
#[derive(Debug, Error)]
pub enum JsonRpcError<E> {
    /// Potential errors returned while sending requests to or receiving responses from the RPC server.
    #[error(transparent)]
    TransportError(RpcTransportError),
    /// Potential errors returned by the RPC server.
    #[error(transparent)]
    ServerError(JsonRpcServerError<E>),
}

impl<E> JsonRpcError<E> {
    pub fn handler_error(&self) -> Option<&E> {
        if let Self::ServerError(JsonRpcServerError::HandlerError(err)) = self {
            return Some(err);
        }
        None
    }
}

impl<E: super::methods::RpcHandlerError> From<RpcError> for JsonRpcError<E> {
    fn from(err: RpcError) -> Self {
        let mut handler_parse_error = None;
        match err.error_struct {
            Some(RpcErrorKind::HandlerError(ref handler_error)) => {
                match E::parse(handler_error.clone()) {
                    Ok(handler_error) => {
                        return JsonRpcError::ServerError(JsonRpcServerError::HandlerError(
                            handler_error,
                        ))
                    }
                    Err(err) => {
                        handler_parse_error.replace(err);
                    }
                }
            }
            Some(RpcErrorKind::RequestValidationError(err)) => {
                return JsonRpcError::ServerError(JsonRpcServerError::RequestValidationError(err));
            }
            Some(RpcErrorKind::InternalError(err)) => {
                return JsonRpcError::ServerError(JsonRpcServerError::InternalError {
                    info: err["info"]["error_message"]
                        .as_str()
                        .map(|info| info.to_string()),
                })
            }
            None => {}
        }
        if let Some(ref raw_err_data) = err.data {
            match E::parse_legacy_error(raw_err_data.clone()) {
                Some(Ok(handler_error)) => {
                    return JsonRpcError::ServerError(JsonRpcServerError::HandlerError(
                        handler_error,
                    ))
                }
                Some(Err(err)) => {
                    handler_parse_error.replace(err);
                }
                None => {}
            }
        }
        if let Some(err) = handler_parse_error {
            return JsonRpcError::TransportError(RpcTransportError::RecvError(
                JsonRpcTransportRecvError::ResponseParseError(
                    JsonRpcTransportHandlerResponseError::ErrorMessageParseError(err),
                ),
            ));
        }
        JsonRpcError::ServerError(JsonRpcServerError::NonContextualError(err))
    }
}
