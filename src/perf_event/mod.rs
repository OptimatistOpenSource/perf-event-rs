mod builder;
mod counting;

pub use builder::*;

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
