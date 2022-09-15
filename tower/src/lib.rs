use std::marker::PhantomData;
use std::pin::Pin;

use core::ops::DerefMut;

pub trait Stream {
    type Item;
}

impl<P> Stream for Pin<P>
where
    P: DerefMut,
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
    type Service;
}

impl<K, S, D: ?Sized> Discover for D
where
    D: TryStream<Ok = (K, S)>,
    K: Eq,
{
    type Service = S;
}

pub trait Service<Request> {
    type Error;
    type Future;
}

pub trait MakeService<Request> {
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

pub struct PoolDiscoverer<MS, Request>
where
    MS: MakeService<Request>,
{
    _p: PhantomData<(MS, Request)>,
}

impl<MS, Request> Stream for PoolDiscoverer<MS, Request>
where
    MS: MakeService<Request>,
{
    type Item = (usize, SvcWrap<MS::Service>);
}

pub struct Builder {}

impl Builder {
    pub fn build<MS, Request>()
    where
        MS: MakeService<Request>,
        MS::Error: Into<crate::BoxError>,
    {
        let d: PoolDiscoverer<MS, Request> = todo!();

        // THE CRITICAL STATEMENT
        let _ = Balance::new(Box::pin(d));
    }
}

pub struct Pool<MS, Request> {
    balance: (MS, Request),
}

impl<MS, Req> Service<Req> for Pool<MS, Req>
where
    MS: MakeService<Req>,
    MS::Error: Into<crate::BoxError>,
{
    type Error = <Balance<PoolDiscoverer<MS, Req>, Req> as Service<Req>>::Error;
    type Future = <Balance<PoolDiscoverer<MS, Req>, Req> as Service<Req>>::Future;
}

pub struct SvcWrap<Svc> {
    svc: Svc,
}

impl<Request, Svc: Service<Request>> Service<Request> for SvcWrap<Svc> {
    type Future = Svc::Future;
    type Error = Svc::Error;
}
