#![allow(warnings)]

#[macro_use]
pub(crate) mod macros;
pub mod balance;

pub mod discover;

pub mod make;


#[allow(unreachable_pub)]
mod sealed {
    pub trait Sealed<T> {}
}

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

mod load {
    pub trait Load {
        type Metric;
        fn load(&self) -> Self::Metric;
    }
}
