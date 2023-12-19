use crate::perf_event::event::tracing::TracingEvent;
use crate::syscall::bindings::*;

pub struct BreakpointEvent {
    pub bp_type: BreakpointType,
}

impl BreakpointEvent {
    pub const fn new(bp_type: BreakpointType) -> Self {
        Self { bp_type }
    }
}

pub enum BreakpointType {
    /*
    Line 582 of kernel/events/hw_breakpoint.c:
    ```c
    /* Basic checks */
    if (bp_type == HW_BREAKPOINT_EMPTY ||
        bp_type == HW_BREAKPOINT_INVALID)
        return -EINVAL;
    ```
    So `HW_BREAKPOINT_EMPTY` is not provided here.
     */
    R { addr: u64, len: BreakpointLen },
    W { addr: u64, len: BreakpointLen },
    Rw { addr: u64, len: BreakpointLen },
    X { addr: u64 },
}

pub enum BreakpointLen {
    Len1,
    Len2,
    Len4,
    Len8,
}

impl BreakpointLen {
    pub(crate) const fn into_u64(self) -> u64 {
        let val = match self {
            Self::Len1 => HW_BREAKPOINT_LEN_1,
            Self::Len2 => HW_BREAKPOINT_LEN_2,
            Self::Len4 => HW_BREAKPOINT_LEN_4,
            Self::Len8 => HW_BREAKPOINT_LEN_8,
        };
        val as u64
    }
}

impl From<BreakpointEvent> for TracingEvent {
    fn from(value: BreakpointEvent) -> Self {
        Self::Breakpoint(value)
    }
}
