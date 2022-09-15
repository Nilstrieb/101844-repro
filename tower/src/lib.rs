use std::marker::PhantomData;
use std::pin::Pin;

use core::ops::DerefMut;

trait Stream {
    type Item;
}

impl<P> Stream for Pin<P>
where
    P: DerefMut,
    P::Target: Stream,
{
    type Item = <P::Target as Stream>::Item;
}

trait TryStream: Stream {
    type Ok;
}

impl<S, T> TryStream for S
where
    S: ?Sized + Stream<Item = T>,
{
    type Ok = T;
}

trait Discover {
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
}

pub trait MakeService {
    type Error;
    type Service: Service<(), Error = Self::Error>;
}

struct Balance<D, Req> {
    _req: PhantomData<(D, Req)>,
}

impl<D, Req> Balance<D, Req>
where
    D: Discover,
    D::Service: Service<Req>,
    <D::Service as Service<Req>>::Error: Into<()>,
{
    fn new(_: D) -> Self {
        todo!()
    }
}

impl<D, Req> Service<Req> for Balance<D, Req> {
    type Error = ();
}

struct PoolDiscoverer<MS>
where
    MS: MakeService,
{
    _x: MS,
}

impl<MS> Stream for PoolDiscoverer<MS>
where
    MS: MakeService,
{
    type Item = (usize, SvcWrap<MS::Service>);
}

pub struct Builder;

impl Builder {
    pub fn build<MS, Request>()
    where
        MS: MakeService,
        MS::Error: Into<()>,
    {
        let d: PoolDiscoverer<MS> = todo!();

        // THE CRITICAL STATEMENT
        let _ = Balance::new(Box::pin(d));
    }
}

struct SvcWrap<Svc>(Svc);

impl<Request, Svc: Service<Request>> Service<Request> for SvcWrap<Svc> {
    type Error = Svc::Error;
}
