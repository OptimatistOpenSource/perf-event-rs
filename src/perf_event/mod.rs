mod builder;
pub mod counting;
pub mod event;
pub mod sampling;
pub mod tracing;

use crate::syscall::bindings::perf_event_attr;
pub use builder::*;
pub use event::*;

type RawAttr = perf_event_attr;

pub enum EventScope {
    User,
    Kernel,
    Hv,
    Idle,
    Host,
    Guest,
    CallchainKernel,
    CallchainUser,
}

impl EventScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::User,
            Self::Kernel,
            Self::Hv,
            Self::Idle,
            Self::Host,
            Self::Guest,
            Self::CallchainKernel,
            Self::CallchainUser,
        ]
    }
}
