pub trait HeaderEntry
where
    Self::HeaderName: reqwest::header::IntoHeaderName,
{
    type HeaderName;

    fn header_name(&self) -> &Self::HeaderName;
    fn header_pair(self) -> (Self::HeaderName, reqwest::header::HeaderValue);
}

impl HeaderEntry for (reqwest::header::HeaderName, reqwest::header::HeaderValue) {
    type HeaderName = reqwest::header::HeaderName;

    fn header_name(&self) -> &Self::HeaderName {
        &self.0
    }

    fn header_pair(self) -> (Self::HeaderName, reqwest::header::HeaderValue) {
        self
    }
}
