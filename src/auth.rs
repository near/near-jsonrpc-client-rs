use std::fmt;

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct ApiKey(reqwest::header::HeaderValue);

impl ApiKey {
    /// Creates a new API key from a string.
    pub fn new<K: IntoHeaderValue>(api_key: K) -> Result<Self, InvalidApiKey> {
        if !api_key
            .as_ref()
            .iter()
            .all(|&b| b == b'-' || b.is_ascii_hexdigit())
        {
            return Err(InvalidApiKey { _priv: () });
        }
        if let Ok(api_key) = api_key.try_into() {
            return Ok(ApiKey(api_key));
        }
        unreachable!()
    }

    pub fn as_str(&self) -> &str {
        if let Ok(s) = self.0.to_str() {
            return s;
        }
        unreachable!()
    }
}

impl crate::headers::HeaderEntry for ApiKey {
    type HeaderName = &'static str;

    fn header_name(&self) -> &Self::HeaderName {
        &"x-api-key"
    }

    fn header_pair(self) -> (Self::HeaderName, reqwest::header::HeaderValue) {
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
