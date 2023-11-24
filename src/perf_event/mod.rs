mod builder;
pub mod counting;
pub mod event;
pub mod sampling;

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
