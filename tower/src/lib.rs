use std::marker::PhantomData;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use core::ops::DerefMut;

pub trait Stream {
    type Item;
}

impl<S: ?Sized + Stream + Unpin> Stream for &mut S {
    type Item = S::Item;
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
    type Error;
}

impl<S, T, E> TryStream for S
where
    S: ?Sized + Stream<Item = Result<T, E>>,
{
    type Ok = T;
    type Error = E;
}

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub trait Discover {
    type Key;
    type Service;
    type Error;
}

impl<K, S, E, D: ?Sized> Discover for D
where
    D: TryStream<Ok = (K, S), Error = E>,
    K: Eq,
{
    type Key = K;
    type Service = S;
    type Error = E;
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
    type Item = Result<(usize, DropNotifyService<MS::Service>), MS::Error>;
}

pub struct Builder {}

impl Builder {
    pub fn build<MS, Target, Request>() -> ()
    where
        MS: MakeService<Target, Request>,
        MS::Error: Into<crate::BoxError>,
    {
        let d: PoolDiscoverer<MS, Target, Request> = todo!();

        // THE CRITICAL STATEMENT
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
    MS::Error: Into<crate::BoxError>,
    Target: Clone,
{
    type Error = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Error;
    type Future = <PinBalance<PoolDiscoverer<MS, Target, Req>, Req> as Service<Req>>::Future;
}

pub struct DropNotifyService<Svc> {
    svc: Svc,
}

impl<Request, Svc: Service<Request>> Service<Request> for DropNotifyService<Svc> {
    type Future = Svc::Future;
    type Error = Svc::Error;
}
