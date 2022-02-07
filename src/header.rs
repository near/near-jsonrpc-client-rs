use std::marker::PhantomData;

use reqwest::header::{HeaderValue, IntoHeaderName};

use super::JsonRpcClient;

pub struct Prevalidated {
    _priv: (),
}

pub struct Postvalidated<E> {
    _priv: PhantomData<E>,
}

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
