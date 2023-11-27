use crate::debug_union;
use crate::syscall::bindings::perf_event_attr__bindgen_ty_2;
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_attr__bindgen_ty_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            name: perf_event_attr__bindgen_ty_2
            self: self
            fmt: f
            fields:
                wakeup_events
                wakeup_watermark
        }

        Ok(())
    }
}
