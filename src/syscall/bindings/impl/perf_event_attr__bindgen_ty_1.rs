use crate::debug_union;
use crate::syscall::bindings::perf_event_attr__bindgen_ty_1;
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_attr__bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            name: perf_event_attr__bindgen_ty_1
            self: self
            fmt: f
            fields:
                sample_period
                sample_freq
        }

        Ok(())
    }
}
