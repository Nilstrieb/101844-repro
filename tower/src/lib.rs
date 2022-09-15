pub mod balance;

use futures_core::TryStream;
use std::future::Future;
pub trait Sealed<T> {}

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

mod load {
    pub trait Load {
        type Metric;
        fn load(&self) -> Self::Metric;
    }
}

pub trait Discover {
    type Key;
    type Service;
    type Error;
}

impl<K, S, E, D: ?Sized> Sealed<Change<(), ()>> for D
where
    D: TryStream<Ok = Change<K, S>, Error = E>,
    K: Eq,
{
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
    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
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
