mod builder;
mod counting;
mod mode;

use std::marker::PhantomData;
use std::os::fd::RawFd;

pub use attr::*;
pub use builder::*;
pub use counting::*;

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

pub struct PerfEvent<M> {
    // TODO
    raw_fd: RawFd,
    phantom: PhantomData<M>,
}
