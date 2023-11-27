use crate::debug_union;
use crate::syscall::bindings::perf_event_attr__bindgen_ty_4;
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_attr__bindgen_ty_4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            name: perf_event_attr__bindgen_ty_4
            self: self
            fmt: f
            fields:
                bp_len
                kprobe_addr
                probe_offset
                config2
        }

        Ok(())
    }
}
