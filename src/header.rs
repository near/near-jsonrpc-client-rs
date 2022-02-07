//! Client Headers.
//!
//! This module includes everything you need to build valid header entries.

use std::marker::PhantomData;

pub use reqwest::header::HeaderValue;
use reqwest::header::IntoHeaderName;

use super::JsonRpcClient;

/// [`HeaderEntry`] attribute identifying those that have been prevalidated.
///
/// The specification of a header entry identified by this discriminant doesn't return a [`Result`].
///
/// ### Example
///
/// This example adds the header name `custom-header` and value `custom:some-value`.
///
/// ```
/// use near_jsonrpc_client::{
///     header::{HeaderEntry, HeaderValue, Prevalidated},
///     methods, JsonRpcClient,
/// };
///
/// struct CustomHeader(HeaderValue);
///
/// impl HeaderEntry<Prevalidated> for CustomHeader {
///     type HeaderName = &'static str;
///     type HeaderValue = HeaderValue;
///
///     fn header_name(&self) -> &Self::HeaderName {
///         &"custom-header"
///     }
///
///     fn header_pair(self) -> (Self::HeaderName, Self::HeaderValue) {
///         ("custom-header", self.0)
///     }
/// }
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///
/// let header_value = HeaderValue::try_from("custom:some-value")?; // <- error handling here
///
/// let client = JsonRpcClient::connect("https://rpc.testnet.near.org").header(CustomHeader(header_value));
/// # Ok(())
/// # }
pub struct Prevalidated {
    _priv: (),
}

/// [`HeaderEntry`] attribute identifying those that need to be validated.
///
/// The specification of a header entry identified by this discriminant will return a [`Result`].
///
/// ### Example
///
/// This example adds the header name `custom-header` and value `custom:some-value`.
///
/// ```
/// # use std::{fmt, error::Error};
/// use near_jsonrpc_client::{
///     header::{HeaderEntry, HeaderValue, Postvalidated},
///     methods, JsonRpcClient,
/// };
///
/// struct CustomValue(&'static str);
///
/// struct CustomHeader(CustomValue);
///
/// # #[derive(Debug)]
/// struct CustomError;
/// # impl Error for CustomError {}
/// # impl fmt::Display for CustomError {
/// #     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
/// #         write!(f, "custom error")
/// #     }
/// # }
///
/// impl TryFrom<CustomValue> for HeaderValue {
///     type Error = CustomError;
///
///     fn try_from(v: CustomValue) -> Result<Self, Self::Error> {
///         HeaderValue::try_from(format!("custom:{}", v.0)).map_err(|_| CustomError)
///     }
/// }
///
/// impl HeaderEntry<Postvalidated<CustomError>> for CustomHeader {
///     type HeaderName = &'static str;
///     type HeaderValue = CustomValue;
///
///     fn header_name(&self) -> &Self::HeaderName {
///         &"custom-header"
///     }
///
///     fn header_pair(self) -> (Self::HeaderName, Self::HeaderValue) {
///         ("custom-header", self.0)
///     }
/// }
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
///
/// let client = JsonRpcClient::connect("https://rpc.testnet.near.org")
///     .header(CustomHeader(CustomValue("some-value")))?; // <- error handling here
/// # Ok(())
/// # }
pub struct Postvalidated<E> {
    _priv: PhantomData<E>,
}

/// Trait for identifying valid header entries.
///
/// Header entries are distinguished by their discrimimants, (See [HeaderEntryDiscriminant]).
pub trait HeaderEntry<D = Prevalidated>: Sized
where
    D: HeaderEntryDiscriminant<Self>,
{
    type HeaderName;
    type HeaderValue;

    fn header_name(&self) -> &Self::HeaderName;
    fn header_pair(self) -> (Self::HeaderName, Self::HeaderValue);
}

mod private {
    pub trait Sealed {}
}

/// Trait for defining a [`HeaderEntry`]'s application on a client.
pub trait HeaderEntryDiscriminant<H>: private::Sealed {
    type Output;

    fn apply(client: JsonRpcClient, entry: H) -> Self::Output;
}

impl private::Sealed for Prevalidated {}
impl<T> HeaderEntryDiscriminant<T> for Prevalidated
where
    T: HeaderEntry<Self, HeaderValue = HeaderValue>,
    T::HeaderName: IntoHeaderName,
{
    type Output = JsonRpcClient;

    fn apply(mut client: JsonRpcClient, entry: T) -> Self::Output {
        let (k, v) = entry.header_pair();
        client.headers.append(k, v);
        client
    }
}

impl<E> private::Sealed for Postvalidated<E> {}
impl<T, E> HeaderEntryDiscriminant<T> for Postvalidated<E>
where
    T: HeaderEntry<Self>,
    T::HeaderName: IntoHeaderName,
    T::HeaderValue: TryInto<HeaderValue, Error = E>,
{
    type Output = Result<JsonRpcClient, E>;

    fn apply(mut client: JsonRpcClient, entry: T) -> Self::Output {
        let (k, v) = entry.header_pair();
        let v = v.try_into()?;
        client.headers.append(k, v);
        Ok(client)
    }
}

impl<N: IntoHeaderName> HeaderEntry<Prevalidated> for (N, HeaderValue) {
    type HeaderName = N;
    type HeaderValue = HeaderValue;

    fn header_name(&self) -> &Self::HeaderName {
        &self.0
    }

    fn header_pair(self) -> (Self::HeaderName, Self::HeaderValue) {
        self
    }
}

impl<N, V> HeaderEntry<Postvalidated<V::Error>> for (N, V)
where
    N: IntoHeaderName,
    V: TryInto<HeaderValue>,
{
    type HeaderName = N;
    type HeaderValue = V;

    fn header_name(&self) -> &Self::HeaderName {
        &self.0
    }

    fn header_pair(self) -> (Self::HeaderName, Self::HeaderValue) {
        self
    }
}
