use super::super::error;
use crate::discover::{Change, Discover};
use crate::load::Load;
use futures_core::ready;
use futures_util::future::{self, TryFutureExt};
use pin_project_lite::pin_project;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::hash::Hash;
use std::marker::PhantomData;
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::oneshot;
use tower_service::Service;
use tracing::{debug, trace};

pub struct Balance<D, Req> {
    _req: PhantomData<(D, Req)>,
}

impl<D, Req> Balance<D, Req>
where
    D: Discover,
    D::Service: Service<Req>,
    <D::Service as Service<Req>>::Error: Into<crate::BoxError>,
{
    pub fn new(discover: D) -> Self {
        todo!()
    }
}

impl<D, Req> Service<Req> for Balance<D, Req>
where
    D: Discover + Unpin,
    D::Key: Hash + Clone,
    D::Error: Into<crate::BoxError>,
    D::Service: Service<Req> + Load,
    <D::Service as Load>::Metric: std::fmt::Debug,
    <D::Service as Service<Req>>::Error: Into<crate::BoxError>,
{
    type Response = <D::Service as Service<Req>>::Response;
    type Error = crate::BoxError;
    type Future = future::MapErr<
        <D::Service as Service<Req>>::Future,
        fn(<D::Service as Service<Req>>::Error) -> crate::BoxError,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, request: Req) -> Self::Future {
        todo!()
    }
}
