use crate::load::Load;
use crate::MakeService;
use crate::Service;
use crate::{Change, Discover};
use futures_core::Stream;
use futures_util::future::{self};
use std::hash::Hash;
use std::marker::PhantomData;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Balance<D, Req> {
    _req: PhantomData<(D, Req)>,
}

impl<D, Req> Balance<D, Req>
where
    D: Discover,
    D::Service: Service<Req>,
    <D::Service as Service<Req>>::Error: Into<crate::BoxError>,
{
    pub fn new(_: D) -> Self {
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
}

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
    type Item = Result<Change<usize, DropNotifyService<MS::Service>>, MS::MakeError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

pub struct Builder {}

impl Builder {
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

pub struct Pool<MS, Target, Request> {
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
}

pub struct DropNotifyService<Svc> {
    svc: Svc,
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

}
