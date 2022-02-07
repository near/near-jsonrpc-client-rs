use std::fmt;

use reqwest::header::HeaderValue;

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct ApiKey(HeaderValue);

impl ApiKey {
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
        &"x-api-key"
    }

    fn header_pair(self) -> (Self::HeaderName, HeaderValue) {
        ("x-api-key", self.0)
    }
}

#[derive(Eq, Clone, Debug, PartialEq)]
pub struct InvalidApiKey {
    _priv: (),
}

impl std::error::Error for InvalidApiKey {}
impl fmt::Display for InvalidApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid API key")
    }
}

mod private {
    pub trait IntoHeaderValueSealed: AsRef<[u8]> + TryInto<reqwest::header::HeaderValue> {}
}

pub trait IntoHeaderValue: private::IntoHeaderValueSealed {}

impl private::IntoHeaderValueSealed for String {}

impl IntoHeaderValue for String {}

impl private::IntoHeaderValueSealed for &String {}

impl IntoHeaderValue for &String {}

impl private::IntoHeaderValueSealed for &str {}

impl IntoHeaderValue for &str {}
