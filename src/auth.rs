pub struct AuthHeaderEntry<'a> {
    pub header: &'a str,
    pub value: &'a str,
}

mod private {
    pub trait AuthState {
        fn maybe_auth_header(&self) -> Option<super::AuthHeaderEntry>;
    }
}

pub trait AuthState: private::AuthState {}

#[derive(Debug, Clone)]
pub struct Unauthenticated;
impl AuthState for Unauthenticated {}
impl private::AuthState for Unauthenticated {
    fn maybe_auth_header(&self) -> Option<AuthHeaderEntry> {
        None
    }
}

pub trait AuthScheme {
    fn get_auth_header(&self) -> AuthHeaderEntry;
}

#[derive(Debug, Clone)]
pub struct Authenticated<T> {
    pub(crate) auth_scheme: T,
}

impl<T: AuthScheme> AuthState for Authenticated<T> {}
impl<T: AuthScheme> private::AuthState for Authenticated<T> {
    fn maybe_auth_header(&self) -> Option<AuthHeaderEntry> {
        Some(self.auth_scheme.get_auth_header())
    }
}

#[derive(Clone, Debug)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self(api_key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AuthScheme for ApiKey {
    fn get_auth_header(&self) -> AuthHeaderEntry {
        AuthHeaderEntry {
            header: "x-api-key",
            value: self.0.as_str(),
        }
    }
}
