//! Contains [`MakeService`] which is a trait alias for a [`Service`] of [`Service`]s.

use crate::sealed::Sealed;
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use tower_service::Service;

pub(crate) mod shared;

/// Creates new [`Service`] values.
///
/// Acts as a service factory. This is useful for cases where new [`Service`]
/// values must be produced. One case is a TCP server listener. The listener
/// accepts new TCP streams, obtains a new [`Service`] value using the
/// [`MakeService`] trait, and uses that new [`Service`] value to process inbound
/// requests on that new TCP stream.
///
/// This is essentially a trait alias for a [`Service`] of [`Service`]s.
pub trait MakeService<Target, Request> {
    type Response;
    type Error;
    type Service: Service<Request, Response = Self::Response, Error = Self::Error>;
    type MakeError;
    type Future;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::MakeError>>;
    fn make_service(&mut self, target: Target) -> Self::Future;
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

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::MakeError>> {
        Service::poll_ready(self, cx)
    }

    fn make_service(&mut self, target: Target) -> Self::Future {
        Service::call(self, target)
    }
}

/// Service returned by [`MakeService::into_service`][into].
///
/// See the documentation on [`into_service`][into] for details.
///
/// [into]: MakeService::into_service
pub struct IntoService<M, Request> {
    make: M,
    _marker: PhantomData<Request>,
}

impl<M, Request> Clone for IntoService<M, Request>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            make: self.make.clone(),
            _marker: PhantomData,
        }
    }
}

impl<M, Request> fmt::Debug for IntoService<M, Request>
where
    M: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntoService")
            .field("make", &self.make)
            .finish()
    }
}

impl<M, S, Target, Request> Service<Target> for IntoService<M, Request>
where
    M: Service<Target, Response = S>,
    S: Service<Request>,
{
    type Response = M::Response;
    type Error = M::Error;
    type Future = M::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.make.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, target: Target) -> Self::Future {
        self.make.make_service(target)
    }
}

/// Service returned by [`MakeService::as_service`][as].
///
/// See the documentation on [`as_service`][as] for details.
///
/// [as]: MakeService::as_service
pub struct AsService<'a, M, Request> {
    make: &'a mut M,
    _marker: PhantomData<Request>,
}

impl<M, Request> fmt::Debug for AsService<'_, M, Request>
where
    M: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsService")
            .field("make", &self.make)
            .finish()
    }
}

impl<M, S, Target, Request> Service<Target> for AsService<'_, M, Request>
where
    M: Service<Target, Response = S>,
    S: Service<Request>,
{
    type Response = M::Response;
    type Error = M::Error;
    type Future = M::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.make.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, target: Target) -> Self::Future {
        self.make.make_service(target)
    }
}
