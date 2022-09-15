pub mod balance;

use futures_core::TryStream;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub trait Discover {
    type Key;
    type Service;
    type Error;
}

impl<K, S, E, D: ?Sized> Discover for D
where
    D: TryStream<Ok = Change<K, S>, Error = E>,
    K: Eq,
{
    type Key = K;
    type Service = S;
    type Error = E;
}

pub struct Change<K, V>(K, V);

pub trait Service<Request> {
    type Response;
    type Error;
    type Future;
}

pub trait MakeService<Target, Request> {
    type Response;
    type Error;
    type Service: Service<Request, Response = Self::Response, Error = Self::Error>;
    type Future;
}
