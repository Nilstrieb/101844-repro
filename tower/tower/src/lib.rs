#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![forbid(unsafe_code)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#![allow(warnings)]

#[macro_use]
pub(crate) mod macros;
#[cfg(feature = "balance")]
#[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
pub mod balance;

#[cfg(feature = "discover")]
#[cfg_attr(docsrs, doc(cfg(feature = "discover")))]
pub mod discover;

#[cfg(feature = "load")]
#[cfg_attr(docsrs, doc(cfg(feature = "load")))]
pub mod load;


#[cfg(feature = "make")]
#[cfg_attr(docsrs, doc(cfg(feature = "make")))]
pub mod make;


pub mod builder;
pub mod layer;

#[doc(inline)]
pub use crate::builder::ServiceBuilder;
#[cfg(feature = "make")]
#[cfg_attr(docsrs, doc(cfg(feature = "make")))]
#[doc(inline)]
pub use crate::make::MakeService;


#[allow(unreachable_pub)]
mod sealed {
    pub trait Sealed<T> {}
}

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
