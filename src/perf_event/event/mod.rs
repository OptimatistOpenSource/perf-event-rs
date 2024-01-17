mod breakpoint;
mod dynamic_pmu;
mod hardware;
mod raw;
mod scope;
mod software;
mod tracepoint;

use crate::infra::SizedExt;
use crate::perf_event::RawAttr;
use crate::syscall::bindings::*;
use libc::c_long;

pub use breakpoint::*;
pub use dynamic_pmu::*;
pub use hardware::*;
pub use raw::*;
pub use scope::*;
pub use software::*;
pub use tracepoint::*;

#[derive(Clone, Debug)]
pub enum Event {
    Hardware(HardwareEvent),
    Software(SoftwareEvent),
    Raw(RawEvent),
    Tracepoint(TracepointEvent),
    Breakpoint(BreakpointEvent),
    DynamicPmu(DynamicPmuEvent),
}
