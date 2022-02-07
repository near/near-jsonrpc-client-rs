use std::fmt;

use reqwest::header::HeaderValue;

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct ApiKey(HeaderValue);

impl ApiKey {
    pub const HEADER_NAME: &'static str = "x-api-key";

    /// Creates a new API key from a string.
    pub fn new<K: IntoHeaderValue>(api_key: K) -> Result<Self, InvalidApiKey> {
        if api_key
            .as_ref()
            .iter()
            .all(|&b| b == b'-' || b.is_ascii_hexdigit())
        {
            if let Ok(api_key) = api_key.try_into() {
                return Ok(ApiKey(api_key));
            }
        }
        Err(InvalidApiKey { _priv: () })
    }

    pub fn as_str(&self) -> &str {
        self.0
            .to_str()
            .expect("fatal: api key should contain only ascii characters")
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

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x-api-key: {}", self.as_str())
    }
}

#[derive(Eq, Clone, PartialEq)]
pub struct InvalidApiKey {
    _priv: (),
}

impl fmt::Debug for InvalidApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("InvalidApiKey")
    }
}

impl std::error::Error for InvalidApiKey {}
impl fmt::Display for InvalidApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid API key")
    }
}

mod private {
    pub trait Sealed: AsRef<[u8]> + TryInto<reqwest::header::HeaderValue> {}
}

pub trait IntoHeaderValue: private::Sealed {}

impl private::Sealed for String {}

impl IntoHeaderValue for String {}

impl private::Sealed for &String {}

impl IntoHeaderValue for &String {}

impl private::Sealed for &str {}

impl IntoHeaderValue for &str {}
