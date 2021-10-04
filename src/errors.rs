use std::{fmt, io};

use thiserror::Error;

use near_jsonrpc_primitives::errors::{RpcError, RpcErrorKind, RpcRequestValidationErrorKind};
use near_jsonrpc_primitives::message::{self, Message};

#[derive(Debug, Error)]
pub enum JsonRpcTransportSendError {
    #[error("error while serializing payload: [{0}]")]
    PayloadSerializeError(io::Error),
    #[error("error while sending payload: [{0}]")]
    PayloadSendError(reqwest::Error),
}

#[derive(Debug, Error)]
pub enum JsonRpcTransportHandlerResponseError {
    #[error("error while parsing method call result: [{0}]")]
    ResultParseError(serde_json::Error),
    #[error("error while parsing method call error message: [{0}]")]
    ErrorMessageParseError(serde_json::Error),
}

#[derive(Debug, Error)]
pub enum JsonRpcTransportRecvError {
    #[error("unexpected server response: [{0:?}]")]
    UnexpectedServerResponse(Message),
    #[error("error while reading response: [{0}]")]
    PayloadRecvError(reqwest::Error),
    #[error("error while parsing server response: [{0:?}]")]
    PayloadParseError(message::Broken),
    #[error(transparent)]
    ResponseParseError(JsonRpcTransportHandlerResponseError),
}

#[derive(Debug, Error)]
pub enum RpcTransportError {
    #[error(transparent)]
    SendError(JsonRpcTransportSendError),
    #[error(transparent)]
    RecvError(JsonRpcTransportRecvError),
}

#[derive(Debug)]
pub enum JsonRpcServerError<E> {
    RequestValidationError(RpcRequestValidationErrorKind),
    HandlerError(E),
    InternalError { info: String },
    NonContextualError { code: i64, message: String },
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for JsonRpcServerError<E> {}

impl<E: fmt::Display> fmt::Display for JsonRpcServerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestValidationError(err) => {
                write!(f, "request validation error: [{:?}]", err)
            }
            Self::HandlerError(err) => write!(f, "handler error: [{}]", err),
            Self::InternalError { info } => write!(f, "internal error: [{}]", info),
            Self::NonContextualError { code, message } => write!(
                f,
                "error response lacks context: {{code = {}}} {{message = {}}}",
                code, message
            ),
        }
    }
}

#[derive(Debug)]
pub enum JsonRpcError<E> {
    TransportError(RpcTransportError),
    ServerError(JsonRpcServerError<E>),
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for JsonRpcError<E> {}

impl<E: fmt::Display> fmt::Display for JsonRpcError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransportError(err) => fmt::Display::fmt(err, f),
            Self::ServerError(err) => fmt::Display::fmt(err, f),
        }
    }
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
            Some(RpcErrorKind::HandlerError(handler_error)) => {
                match serde_json::from_value(handler_error) {
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
                        .unwrap_or("<no data>")
                        .to_string(),
                })
            }
            None => {}
        }
        if let Some(raw_err_data) = err.data {
            match E::parse_raw_error(raw_err_data) {
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
        return JsonRpcError::ServerError(JsonRpcServerError::NonContextualError {
            code: err.code,
            message: err.message,
        });
    }
}
