#[derive(Eq, Clone, Debug, PartialEq)]
pub enum ClientCredentials<'a> {
    Basic(&'a str),
}

mod r#priv {
    pub trait Private {}
}

pub trait AuthState: r#priv::Private {
    fn as_credentials(&self) -> Option<ClientCredentials>;
}

pub trait AuthType: r#priv::Private {
    fn as_credentials(&self) -> ClientCredentials;
}

#[derive(Debug, Clone)]
pub struct Unauthenticated;
impl r#priv::Private for Unauthenticated {}
impl AuthState for Unauthenticated {
    fn as_credentials(&self) -> Option<ClientCredentials> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct BasicAuth(pub(crate) String);
impl r#priv::Private for BasicAuth {}
impl AuthType for BasicAuth {
    fn as_credentials(&self) -> ClientCredentials {
        ClientCredentials::Basic(&self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Authenticated<T>(pub(crate) T);
impl<T> r#priv::Private for Authenticated<T> {}
impl<T: AuthType> AuthState for Authenticated<T> {
    fn as_credentials(&self) -> Option<ClientCredentials> {
        Some(self.0.as_credentials())
    }
}
