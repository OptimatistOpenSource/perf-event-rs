use crate::perf_event::event::Event;
use crate::syscall::bindings::*;

#[derive(Clone, Debug)]
pub struct BreakpointEvent {
    pub bp_type: BreakpointType,
}

impl BreakpointEvent {
    pub const fn new(bp_type: BreakpointType) -> Self {
        Self { bp_type }
    }
}

#[derive(Clone, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BreakpointLen {
    Len1,
    Len2,
    // Len{3,5,6,7}: https://github.com/torvalds/linux/commit/651be3cb085341a21847e47c694c249c3e1e4e5b
    #[cfg(feature = "linux-4.10")]
    Len3,
    Len4,
    #[cfg(feature = "linux-4.10")]
    Len5,
    #[cfg(feature = "linux-4.10")]
    Len6,
    #[cfg(feature = "linux-4.10")]
    Len7,
    Len8,
}

impl BreakpointLen {
    pub(crate) const fn as_u64(&self) -> u64 {
        let val = match self {
            Self::Len1 => HW_BREAKPOINT_LEN_1,
            Self::Len2 => HW_BREAKPOINT_LEN_2,
            #[cfg(feature = "linux-4.10")]
            Self::Len3 => HW_BREAKPOINT_LEN_3,
            Self::Len4 => HW_BREAKPOINT_LEN_4,
            #[cfg(feature = "linux-4.10")]
            Self::Len5 => HW_BREAKPOINT_LEN_5,
            #[cfg(feature = "linux-4.10")]
            Self::Len6 => HW_BREAKPOINT_LEN_6,
            #[cfg(feature = "linux-4.10")]
            Self::Len7 => HW_BREAKPOINT_LEN_7,
            Self::Len8 => HW_BREAKPOINT_LEN_8,
        };
        val as u64
    }
}

impl From<BreakpointEvent> for Event {
    fn from(value: BreakpointEvent) -> Self {
        Self::Breakpoint(value)
    }
}
