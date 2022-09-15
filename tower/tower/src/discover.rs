use crate::Sealed;
use futures_core::TryStream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

mod error {
    pub enum Never {}
}

pub trait Discover {
    type Key: Eq;
    type Service;
    type Error;
    fn poll_discover(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> (Poll<Option<Result<Change<Self::Key, Self::Service>, Self::Error>>>);
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

    fn poll_discover(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<D::Ok, D::Error>>> {
        todo!()
    }
}

/// A change in the service set.
#[derive(Debug)]
pub enum Change<K, V> {
    Insert(K, V),
    Remove(K),
}
