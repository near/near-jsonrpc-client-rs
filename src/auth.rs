//! Helpers for client authentication.
//!
//! Some RPC nodes will require authentication before requests can be sent to them.
//!
//! This module provides the [`ApiKey`] and [`Authorization`] types that can be used to authenticate
//! requests.
//!
//! ## Example
//!
//! ### API Key (`x-api-key` Header)
//!
//! ```
//! use near_jsonrpc_client::{JsonRpcClient, auth, methods};
//! use near_primitives::types::{BlockReference, Finality};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://rpc.testnet.near.org");
//!
//! let client = client.header(auth::ApiKey::new("399ba741-e939-4ffa-8c3c-306ec36fa8de")?);
//!
//! let request = methods::block::RpcBlockRequest {
//!     block_reference: BlockReference::Finality(Finality::Final),
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(response, methods::block::RpcBlockResponse { .. }));
//! # Ok(())
//! # }
//! ```
//!
//! ### `Authorization` Header
//!
//! ```
//! use near_jsonrpc_client::{JsonRpcClient, auth, methods};
//! use near_primitives::types::{BlockReference, Finality};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://rpc.testnet.near.org")
//!     .header(auth::Authorization::bearer(
//!         "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
//!     )?);
//!
//! let request = methods::block::RpcBlockRequest {
//!     block_reference: BlockReference::Finality(Finality::Final),
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(response, methods::block::RpcBlockResponse { .. }));
//! # Ok(())
//! # }
//! ```

use std::ops::{Index, RangeFrom};
use std::str;

use super::header::{HeaderValue, InvalidHeaderValue, ToStrError};

/// NEAR JSON RPC API key.
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct ApiKey(HeaderValue);

impl ApiKey {
    pub const HEADER_NAME: &'static str = "x-api-key";

    /// Creates a new API key.
    ///
    /// See the [`auth`](self) module documentation for more information.
    pub fn new<K: AsRef<[u8]>>(api_key: K) -> Result<Self, InvalidHeaderValue> {
        HeaderValue::from_bytes(api_key.as_ref()).map(|mut api_key| {
            ApiKey({
                api_key.set_sensitive(true);
                api_key
            })
        })
    }

    /// Returns a string slice if the API Key only contains visible ASCII chars.
    pub fn to_str(&self) -> Result<&str, ToStrError> {
        self.0.to_str()
    }

    /// Returns the API key as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl crate::header::HeaderEntry for ApiKey {
    type HeaderName = &'static str;
    type HeaderValue = HeaderValue;

    fn header_name(&self) -> &Self::HeaderName {
        &Self::HEADER_NAME
    }

    fn header_pair(self) -> (Self::HeaderName, Self::HeaderValue) {
        (Self::HEADER_NAME, self.0)
    }
}

/// HTTP authorization scheme.
#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum AuthorizationScheme {
    Bearer,
}

/// NEAR JSON RPC authorization header.
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Authorization(AuthorizationScheme, HeaderValue);

impl Authorization {
    pub const HEADER_NAME: &'static str = "Authorization";

    /// Creates a new authorization token with the bearer scheme.
    ///
    /// This does not perform any token-specific validation on the token.
    ///
    /// See the [`auth`](self) module documentation for more information.
    pub fn bearer<T: AsRef<str>>(token: T) -> Result<Self, InvalidHeaderValue> {
        HeaderValue::from_bytes(&[b"Bearer ", token.as_ref().as_bytes()].concat()).map(
            |mut token| {
                Authorization(AuthorizationScheme::Bearer, {
                    token.set_sensitive(true);
                    token
                })
            },
        )
    }

    /// Returns the scheme of the authorization header.
    pub fn scheme(&self) -> AuthorizationScheme {
        self.0
    }

    /// Returns the token as a string slice.
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Returns the token as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.strip_scheme(self.1.as_bytes())
    }

    fn strip_scheme<'a, T: Index<RangeFrom<usize>> + ?Sized>(&self, token: &'a T) -> &'a T::Output {
        &token[match self.0 {
            AuthorizationScheme::Bearer => 7,
        }..]
    }
}

impl crate::header::HeaderEntry for Authorization {
    type HeaderName = &'static str;
    type HeaderValue = HeaderValue;

    fn header_name(&self) -> &Self::HeaderName {
        &Self::HEADER_NAME
    }

    fn header_pair(self) -> (Self::HeaderName, Self::HeaderValue) {
        (Self::HEADER_NAME, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_debug() {
        let api_key = ApiKey::new("this is a very secret secret").expect("valid API key");

        assert_eq!(format!("{:?}", api_key), "ApiKey(Sensitive)");

        assert_eq!(
            api_key.to_str().expect("valid utf8 secret"),
            "this is a very secret secret"
        );

        assert_eq!(api_key.as_bytes(), b"this is a very secret secret");
    }

    #[test]
    fn bearer_token() {
        let token = Authorization::bearer("this is a very secret token").expect("valid token");

        assert_eq!(format!("{:?}", token), "Authorization(Bearer, Sensitive)");

        assert_eq!(token.scheme(), AuthorizationScheme::Bearer);

        assert_eq!(token.as_str(), "this is a very secret token");

        assert_eq!(token.as_bytes(), b"this is a very secret token");
    }
}
