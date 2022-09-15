use std::marker::PhantomData;
use std::pin::Pin;

use core::ops::DerefMut;

pub trait Stream {
    type Item;
}

impl<P> Stream for Pin<P>
where
    P: DerefMut + Unpin,
    P::Target: Stream,
{
    type Item = <P::Target as Stream>::Item;
}

pub trait TryStream: Stream {
    type Ok;
}

impl<S, T> TryStream for S
where
    S: ?Sized + Stream<Item = T>,
{
    type Ok = T;
}

pub type BoxError = ();

pub trait Discover {
    type Key;
    type Service;
    type Error;
}

impl<K, S, D: ?Sized> Discover for D
where
    D: TryStream<Ok = (K, S)>,
    K: Eq,
{
    type Key = K;
    type Service = S;
    type Error = ();
}

pub trait Service<Request> {
    type Error;
    type Future;
}

pub trait MakeService<Target, Request> {
    type Response;
    type Error;
    type Service: Service<Request, Error = Self::Error>;
    type Future;
}

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

impl<D, Req> Service<Req> for Balance<D, Req> {
    type Error = crate::BoxError;
    type Future = std::future::Ready<()>;
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
    type Item = (usize, DropNotifyService<MS::Service>);
}

pub struct Builder {}

impl Builder {
    pub fn build<MS, Target, Request>()
    where
        MS: MakeService<Target, Request>,
        MS::Error: Into<crate::BoxError>,
    {
        let d: PoolDiscoverer<MS, Target, Request> = todo!();

        // THE CRITICAL STATEMENT
        let _ =  Balance::new(Box::pin(d));
    }
}

pub struct Pool<MS, Target, Request> {
    balance: (MS, Target, Request),
}

impl<MS, Target, Req> Service<Req> for Pool<MS, Target, Req>
where
    MS: MakeService<Target, Req>,
    MS::Error: Into<crate::BoxError>,
    Target: Clone,
{
    type Error = <Balance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Error;
    type Future = <Balance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Future;
}

pub struct DropNotifyService<Svc> {
    svc: Svc,
}

impl<Request, Svc: Service<Request>> Service<Request> for DropNotifyService<Svc> {
    type Future = Svc::Future;
    type Error = Svc::Error;
}
