use crate::debug_union;
use crate::syscall::bindings::perf_sample_weight;
use std::fmt::{Debug, Formatter};

impl Debug for perf_sample_weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            name: perf_sample_weight
            self: self
            fmt: f
            fields:
                full
                __bindgen_anon_1
        }

        Ok(())
    }
}
