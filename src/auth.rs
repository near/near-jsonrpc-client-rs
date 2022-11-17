use super::header::{HeaderValue, IntoHeaderValue, InvalidHeaderValue, ToStrError};

/// NEAR JSON RPC API key.
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct ApiKey(HeaderValue);

impl ApiKey {
    pub const HEADER_NAME: &'static str = "x-api-key";

    /// Creates a new API key.
    pub fn new<K: IntoHeaderValue>(api_key: K) -> Result<Self, InvalidHeaderValue> {
        api_key.to_header_value().map(ApiKey)
    }

    /// Returns a `&str` slice if the API Key only contains visible ASCII chars.
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
