use crate::debug_struct;
use crate::syscall::bindings::perf_event_mmap_page;
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_mmap_page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_struct! {
            name: perf_event_mmap_page
            self: self
            fmt: f
            fields:
                version
                compat_version
                lock
                index
                offset
                time_enabled
                time_running
                __bindgen_anon_1
                pmc_width
                time_shift
                time_mult
                time_offset
                time_zero
                size
                time_cycles
                time_mask
                data_head
                data_tail
                #[cfg(feature = "linux-4.1")]
                data_offset
                #[cfg(feature = "linux-4.1")]
                data_size
                aux_head
                aux_tail
                aux_offset
                aux_size
        }

        Ok(())
    }
}
