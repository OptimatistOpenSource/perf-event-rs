#![warn(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]

mod infra;
mod perf_event;
mod syscall;

#[allow(dead_code)]
mod test;

pub use perf_event::*;
