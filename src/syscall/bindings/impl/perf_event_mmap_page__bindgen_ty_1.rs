use crate::debug_union;
use crate::syscall::bindings::perf_event_mmap_page__bindgen_ty_1;
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_mmap_page__bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            name: perf_event_mmap_page__bindgen_ty_1
            self: self
            fmt: f
            fields:
                capabilities
                __bindgen_anon_1
        }

        Ok(())
    }
}
