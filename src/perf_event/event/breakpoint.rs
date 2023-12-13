use crate::syscall::bindings::{
    HW_BREAKPOINT_LEN_1, HW_BREAKPOINT_LEN_2, HW_BREAKPOINT_LEN_4, HW_BREAKPOINT_LEN_8,
};
use crate::TracingEvent;

pub struct BreakpointEvent {
    pub bp_type: BreakpointType,
}

pub enum BreakpointType {
    Empty,
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
