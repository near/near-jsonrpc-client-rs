#[derive(Eq, Clone, Debug, PartialEq)]
pub enum ClientCredentials<'a> {
    Basic(&'a str),
}

mod private {
    pub trait AuthState {
        fn maybe_credentials(&self) -> Option<super::ClientCredentials>;
    }
}

pub trait AuthState: private::AuthState {}

#[derive(Debug, Clone)]
pub struct Unauthenticated;
impl AuthState for Unauthenticated {}
impl private::AuthState for Unauthenticated {
    fn maybe_credentials(&self) -> Option<ClientCredentials> {
        None
    }
}

pub trait AuthScheme {
    fn credentials(&self) -> ClientCredentials;
}

#[derive(Debug, Clone)]
pub struct Authenticated<T>(pub(crate) T);
impl<T: AuthScheme> AuthState for Authenticated<T> {}
impl<T: AuthScheme> private::AuthState for Authenticated<T> {
    fn maybe_credentials(&self) -> Option<ClientCredentials> {
        Some(self.0.credentials())
    }
}

#[derive(Debug, Clone)]
pub struct BasicAuth(pub String);
impl AuthScheme for BasicAuth {
    fn credentials(&self) -> ClientCredentials {
        ClientCredentials::Basic(&self.0)
    }
}
