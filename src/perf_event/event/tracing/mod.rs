mod breakpoint;
mod dynamic_pmu;
mod tracepoint;

pub use breakpoint::*;
pub use dynamic_pmu::*;
pub use tracepoint::*;

#[derive(Clone, Debug)]
pub enum TracingEvent {
    Tracepoint(TracepointEvent),
    Breakpoint(BreakpointEvent),
    DynamicPmu(DynamicPmuEvent),
}
