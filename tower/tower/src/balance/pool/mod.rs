//! This module defines a load-balanced pool of services that adds new services when load is high.
//!
//! The pool uses `poll_ready` as a signal indicating whether additional services should be spawned
//! to handle the current level of load. Specifically, every time `poll_ready` on the inner service
//! returns `Ready`, [`Pool`] consider that a 0, and every time it returns `Pending`, [`Pool`]
//! considers it a 1. [`Pool`] then maintains an [exponential moving
//! average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average) over those
//! samples, which gives an estimate of how often the underlying service has been ready when it was
//! needed "recently" (see [`Builder::urgency`]). If the service is loaded (see
//! [`Builder::loaded_above`]), a new service is created and added to the underlying [`Balance`].
//! If the service is underutilized (see [`Builder::underutilized_below`]) and there are two or
//! more services, then the latest added service is removed. In either case, the load estimate is
//! reset to its initial value (see [`Builder::initial`] to prevent services from being rapidly
//! added or removed.
#![deny(missing_docs)]

use super::p2c::Balance;
use crate::discover::Change;
use crate::load::Load;
use crate::make::MakeService;
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use slab::Slab;
use std::{
    fmt,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

/// A wrapper around `MakeService` that discovers a new service when load is high, and removes a
/// service when load is low. See [`Pool`].
pub struct PoolDiscoverer<MS, Target, Request>
where
    MS: MakeService<Target, Request>,
{
    _p: PhantomData<(MS, Target, Request)>,
}

impl<MS, Target, Request> Stream for PoolDiscoverer<MS, Target, Request>
where
    MS: MakeService<Target, Request>,
{
    type Item = Result<(Change<usize, DropNotifyService<MS::Service>>), MS::MakeError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

/// A [builder] that lets you configure how a [`Pool`] determines whether the underlying service is
/// loaded or not. See the [module-level documentation](self) and the builder's methods for
/// details.
///
///  [builder]: https://rust-lang-nursery.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
#[derive(Copy, Clone, Debug)]
pub struct Builder {}

impl Builder {
    /// See [`Pool::new`].
    pub fn build<MS, Target, Request>() -> ()
    where
        MS: MakeService<Target, Request>,
        MS::Service: Load,
        <MS::Service as Load>::Metric: std::fmt::Debug,
        MS::MakeError: Into<crate::BoxError>,
        MS::Error: Into<crate::BoxError>,
    {
        let d: PoolDiscoverer<MS, Target, Request> = todo!();

        let x = Balance::new(Box::pin(d));

        todo!()
    }
}

/// A dynamically sized, load-balanced pool of `Service` instances.
pub struct Pool<MS, Target, Request> {
    // the Pin<Box<_>> here is needed since Balance requires the Service to be Unpin
    balance: (MS, Target, Request),
}

type PinBalance<S, Request> = Balance<Pin<Box<S>>, Request>;

impl<MS, Target, Req> Service<Req> for Pool<MS, Target, Req>
where
    MS: MakeService<Target, Req>,
    MS::Service: Load,
    <MS::Service as Load>::Metric: std::fmt::Debug,
    MS::MakeError: Into<crate::BoxError>,
    MS::Error: Into<crate::BoxError>,
    Target: Clone,
{
    type Response = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Response;
    type Error = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Error;
    type Future = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Req) -> Self::Future {
        todo!()
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct DropNotifyService<Svc> {
    svc: Svc,
    id: usize,
    notify: tokio::sync::mpsc::UnboundedSender<usize>,
}

impl<Svc> Drop for DropNotifyService<Svc> {
    fn drop(&mut self) {
        todo!()
    }
}

impl<Svc: Load> Load for DropNotifyService<Svc> {
    type Metric = Svc::Metric;
    fn load(&self) -> Self::Metric {
        todo!()
    }
}

impl<Request, Svc: Service<Request>> Service<Request> for DropNotifyService<Svc> {
    type Response = Svc::Response;
    type Future = Svc::Future;
    type Error = Svc::Error;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Request) -> Self::Future {
        todo!()
    }
}
