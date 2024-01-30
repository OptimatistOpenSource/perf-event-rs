pub mod counting;
pub mod event;
pub mod sampling;
pub mod tracing;
pub mod config;

use crate::syscall::bindings::perf_event_attr;
pub use event::*;

type RawAttr = perf_event_attr;
