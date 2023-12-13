mod breakpoint;
mod dynamic_pmu;
mod hw;
mod raw;
mod sw;
mod tracepoint;

pub use breakpoint::*;
pub use dynamic_pmu::*;
pub use hw::*;
pub use raw::*;
pub use sw::*;
pub use tracepoint::*;

pub enum Event {
    Hw(HwEvent),
    Sw(SwEvent),
    Raw(RawEvent),
}

pub enum TracingEvent {
    Tracepoint(TracepointEvent),
    Breakpoint(BreakpointEvent),
    DynamicPmu(DynamicPmuEvent),
}
