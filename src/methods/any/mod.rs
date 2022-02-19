use super::*;

use std::marker::PhantomData;

pub fn request<T: AnyRequestResult>(
    method_name: &str,
    params: serde_json::Value,
) -> RpcAnyRequest<T::Response, T::Error>
where
    T::Response: RpcHandlerResponse,
    T::Error: RpcHandlerError,
{
    RpcAnyRequest {
        method: method_name.to_string(),
        params,
        _data: PhantomData,
    }
}

#[derive(Debug)]
pub struct RpcAnyRequest<T, E> {
    pub method: String,
    pub params: serde_json::Value,
    pub(crate) _data: PhantomData<(T, E)>,
}

impl<T, E> private::Sealed for RpcAnyRequest<T, E> {}

impl<T, E> RpcMethod for RpcAnyRequest<T, E>
where
    T: RpcHandlerResponse,
    E: RpcHandlerError,
{
    type Response = T;
    type Error = E;

    #[inline(always)]
    fn method_name(&self) -> &str {
        &self.method
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(self.params.clone())
    }
}

pub trait AnyRequestResult {
    type Response;
    type Error;
}

impl<T, E> AnyRequestResult for Result<T, E> {
    type Response = T;
    type Error = E;
}

impl<T: RpcMethod> AnyRequestResult for T {
    type Response = T::Response;
    type Error = T::Error;
}
