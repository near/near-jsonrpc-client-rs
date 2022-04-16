//! Error types.
use std::io;

use thiserror::Error;

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind, RpcRequestValidationErrorKind};
use near_jsonrpc_primitives::message::{self, Message};

/// Errors returned while sending a request to the rpc server.
#[derive(Debug, Error)]
pub enum JsonRpcTransportSendError {
    /// An error returned when the client is unable to serialize the request payload before sending it to the server.
    #[error("error while serializing payload: [{0}]")]
    PayloadSerializeError(io::Error),
    /// An error returned when the client is unable to send the request to the server.
    #[error("error while sending payload: [{0}]")]
    PayloadSendError(reqwest::Error),
}

/// Errors returned when the client has an issue parsing the response of a method call.
#[derive(Debug, Error)]
pub enum JsonRpcTransportHandlerResponseError {
    /// An error returned when the client is unable to parse the result of a method call.
    #[error("error while parsing method call result: [{0}]")]
    ResultParseError(serde_json::Error),
    /// An error returned when the client is unable to parse the error message returned when a problematic method call is made.
    #[error("error while parsing method call error message: [{0}]")]
    ErrorMessageParseError(serde_json::Error),
}

/// Errors returned while receiving responses from an rpc server.
#[derive(Debug, Error)]
pub enum JsonRpcTransportRecvError {
    /// An error returned when the receives an error that does not fit any of the known error types.
    #[error("unexpected server response: [{0:?}]")]
    UnexpectedServerResponse(Message),
    /// An error returned when the client is unable to read the response from the rpc server.
    #[error("error while reading response: [{0}]")]
    PayloadRecvError(reqwest::Error),
    /// An error returned when the base response structure is malformed e.g. meta properties like rpc version are missing.
    #[error("error while parsing server response: [{0:?}]")]
    PayloadParseError(message::Broken),
    /// An error returned when the internal response structure is malformed, e.g. block data in a block method response is missing.
    #[error(transparent)]
    ResponseParseError(JsonRpcTransportHandlerResponseError),
}

/// Errors returned while sending requests to or receiving responses from the rpc server.
#[derive(Debug, Error)]
pub enum RpcTransportError {
    /// Errors returned while sending a request to the rpc node.
    #[error(transparent)]
    SendError(JsonRpcTransportSendError),
    /// Errors returned while receiving a response from an rpc server.
    #[error(transparent)]
    RecvError(JsonRpcTransportRecvError),
}

/// Meta errors returned by the rpc server.
#[derive(Debug, Error)]
pub enum JsonRpcServerResponseStatusError {
    /// An error returned when the rpc client is unauthorized.
    #[error("this client is unauthorized")]
    Unauthorized,
    /// An error returned when the rpc client exceeds the rate limit by sending too many requests.
    #[error("this client has exceeded the rate limit")]
    TooManyRequests,
    /// An error returned when the rpc server returns a non-200 status code.
    #[error("the server returned a non-OK (200) status code: [{status}]")]
    Unexpected { status: reqwest::StatusCode },
}

/// Errors returned by the rpc server.
#[derive(Debug, Error)]
pub enum JsonRpcServerError<E> {
    /// An error returned when the rpc server is unable to validation a client's request.
    #[error("request validation error: [{0:?}]")]
    RequestValidationError(RpcRequestValidationErrorKind),
    /// An error returned when the server cannot process a request from an rpc client.
    #[error("handler error: [{0}]")]
    HandlerError(E),
    /// An error returned when the rpc server returns an internal server error.
    #[error("internal error: [{info:?}]")]
    InternalError { info: Option<String> },
    /// An error returned when the rpc server returns a response without context i.e. a response the client doesn't understand.
    #[error("error response lacks context: {0}")]
    NonContextualError(RpcError),
    /// Meta errors returned by the rpc server.
    #[error(transparent)]
    ResponseStatusError(JsonRpcServerResponseStatusError),
}

/// Errors returned by the rpc client.
#[derive(Debug, Error)]
pub enum JsonRpcError<E> {
    /// These errors are returned when there's a problem sending a request to an rpc server.
    #[error(transparent)]
    TransportError(RpcTransportError),
    /// These errors are returned whenerver there's a problem with the rpc server.
    #[error(transparent)]
    ServerError(JsonRpcServerError<E>),
}

impl<E> JsonRpcError<E> {
    pub fn handler_error(self) -> Result<E, Self> {
        match self {
            Self::ServerError(JsonRpcServerError::HandlerError(err)) => Ok(err),
            err => Err(err),
        }
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
            match E::parse_raw_error(raw_err_data.clone()) {
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
