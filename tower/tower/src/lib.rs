// #![allow(warnings)]

pub mod balance;
pub mod discover;
pub mod make;

pub trait Sealed<T> {}

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

mod load {
    pub trait Load {
        type Metric;
        fn load(&self) -> Self::Metric;
    }
}

use std::future::Future;
use std::task::Poll;

pub trait Service<Request> {
    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<Result<(), Self::Error>>;

    fn call(&mut self, req: Request) -> Self::Future;
}
