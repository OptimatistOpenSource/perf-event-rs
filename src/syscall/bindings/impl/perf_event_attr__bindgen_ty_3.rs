use crate::debug_union;
use crate::syscall::bindings::perf_event_attr__bindgen_ty_3;
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_attr__bindgen_ty_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            name: perf_event_attr__bindgen_ty_3
            self: self
            fmt: f
            fields:
                bp_addr
                #[cfg(feature = "linux-4.17")]
                kprobe_func
                #[cfg(feature = "linux-4.17")]
                uprobe_path
                config1
        }

        Ok(())
    }
}
