//! Trait aliases for Services that produce specific types of Responses.

pub use self::make_service::shared::Shared;
pub use self::make_service::MakeService;

pub mod future {
    //! Future types

    pub use super::make_service::shared::SharedFuture;
}

mod make_service {
    //! Contains [`MakeService`] which is a trait alias for a [`Service`] of [`Service`]s.

    use crate::sealed::Sealed;
    use std::fmt;
    use std::future::Future;
    use std::marker::PhantomData;
    use std::task::{Context, Poll};
        use crate::Service;

    pub mod shared {
        use std::convert::Infallible;
        use std::task::{Context, Poll};
        use crate::Service;

        pub struct Shared<S> {
            service: S,
        }

        impl<S, T> Service<T> for Shared<S>
        where
            S: Clone,
        {
            type Response = S;
            type Error = Infallible;
            type Future = SharedFuture<S>;

            fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                todo!()
            }

            fn call(&mut self, _target: T) -> Self::Future {
                todo!()
            }
        }

        opaque_future! {
            /// Response future from [`Shared`] services.
            pub type SharedFuture<S> = futures_util::future::Ready<Result<S, Infallible>>;
        }
    }

    pub trait MakeService<Target, Request> {
        type Response;
        type Error;
        type Service: Service<Request, Response = Self::Response, Error = Self::Error>;
        type MakeError;
        type Future;
        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::MakeError>>;
        fn make_service(&mut self, target: Target) -> Self::Future;
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

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::MakeError>> {
            Service::poll_ready(self, cx)
        }

        fn make_service(&mut self, target: Target) -> Self::Future {
            Service::call(self, target)
        }
    }
}
