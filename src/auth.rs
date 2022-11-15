use std::{fmt, str};

use reqwest::header::HeaderValue;

/// NEAR JSON RPC API key.
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct ApiKey(HeaderValue);

impl ApiKey {
    pub const HEADER_NAME: &'static str = "x-api-key";

    /// Creates a new API key from a string.
    pub fn new<K: IntoApiKey>(api_key: K) -> Result<Self, InvalidApiKey> {
        if let Ok(api_key) = uuid::Uuid::try_parse(api_key.as_ref()) {
            if let Ok(api_key) = api_key.to_string().try_into() {
                return Ok(ApiKey(api_key));
            }
        }
        Err(InvalidApiKey { _priv: () })
    }

    /// Returns the API key as a string slice.
    pub fn as_str(&self) -> &str {
        // SAFETY: `HeaderValue` is guaranteed to be a valid UTF-8 string.
        unsafe { str::from_utf8_unchecked(self.0.as_bytes()) }
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

/// An error returned when an API key contains invalid characters.
#[derive(Eq, Clone, PartialEq)]
pub struct InvalidApiKey {
    _priv: (),
}

impl fmt::Debug for InvalidApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("InvalidApiKey")
    }
}

impl std::error::Error for InvalidApiKey {}
impl fmt::Display for InvalidApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Invalid API key")
    }
}

mod private {
    pub trait Sealed: AsRef<str> {}
}

/// A marker trait used to identify values that can be made into API keys.
pub trait IntoApiKey: private::Sealed {}

impl private::Sealed for String {}

impl IntoApiKey for String {}

impl private::Sealed for &String {}

impl IntoApiKey for &String {}

impl private::Sealed for &str {}

impl IntoApiKey for &str {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        ApiKey::new("some-value").expect_err("should not have been a valid API key");

        ApiKey::new("0ee1872b-355f-4254-8e2b-1c0b8199ee92")
            .expect("should have been a valid API key");

        ApiKey::new("0ee1872b355f42548e2b1c0b8199ee92").expect("should have been a valid API key");

        ApiKey::new("0ee--1872b355f4254-8e2b1c0b8-199ee92")
            .expect_err("should not have been a valid API key");
    }

    #[test]
    fn as_str() {
        let api_key = ApiKey::new("0ee1872b-355f-4254-8e2b-1c0b8199ee92")
            .expect("should have been a valid API key");

        assert_eq!(api_key.as_str(), "0ee1872b-355f-4254-8e2b-1c0b8199ee92");

        let api_key = ApiKey::new("0ee1872b355f42548e2b1c0b8199ee92")
            .expect("should have been a valid API key");

        assert_eq!(api_key.as_str(), "0ee1872b-355f-4254-8e2b-1c0b8199ee92");
    }
}
