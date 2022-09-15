pub mod completion;
pub mod peak_ewma;
pub mod pending_requests;

pub use self::{
    completion::{CompleteOnResponse, TrackCompletion},
    peak_ewma::PeakEwma,
    pending_requests::PendingRequests,
};

#[cfg(feature = "discover")]
pub use self::{peak_ewma::PeakEwmaDiscover, pending_requests::PendingRequestsDiscover};

pub trait Load {
    type Metric: PartialOrd;
    fn load(&self) -> Self::Metric;
}
