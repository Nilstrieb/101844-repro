//! Trait aliases for Services that produce specific types of Responses.

use crate::Sealed;
use crate::Service;

use std::convert::Infallible;

pub struct Shared<S> {
    service: S,
}

impl<S, T> Service<T> for Shared<S>
where
    S: Clone,
{
    type Response = S;
    type Error = Infallible;
    type Future = SharedFuture<S>;
}

pub struct SharedFuture<S> {
    _s: S,
}

impl<S> std::future::Future for SharedFuture<S>
where
    std::future::Ready<Result<S, Infallible>>: std::future::Future,
{
    type Output =
        <std::future::Ready<Result<S, Infallible>> as std::future::Future>::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

pub trait MakeService<Target, Request> {
    type Response;
    type Error;
    type Service: Service<Request, Response = Self::Response, Error = Self::Error>;
    type MakeError;
    type Future;
}

impl<M, S, Target, Request> Sealed<(Target, Request)> for M
where
    M: Service<Target, Response = S>,
    S: Service<Request>,
{
}

impl<M, S, Target, Request> MakeService<Target, Request> for M
where
    M: Service<Target, Response = S>,
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Service = S;
    type MakeError = M::Error;
    type Future = M::Future;
}
