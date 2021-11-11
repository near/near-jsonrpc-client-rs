use near_primitives::serialize::to_base64;

pub struct AuthHeaderEntry {
    pub header: String,
    pub value: String,
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

#[derive(Debug, Clone)]
pub enum ApiKey {
    Plain(String),
    Base64(String),
}

impl AuthScheme for ApiKey {
    fn get_auth_header(&self) -> AuthHeaderEntry {
        AuthHeaderEntry {
            header: "x-api-key".to_string(),
            value: match self {
                ApiKey::Plain(ref token) => to_base64(token),
                ApiKey::Base64(ref token) => token.clone(),
            },
        }
    }
}
