use crate::syscall::bindings::perf_event_attr;
use crate::{debug_struct, debug_struct_fn};
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_attr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_struct! {
            name: perf_event_attr
            self: self
            fmt: f
            fields:
                type_
                size
                config
                __bindgen_anon_1
                sample_type
                read_format
                _bitfield_1
                __bindgen_anon_2
                bp_type
                __bindgen_anon_3
                __bindgen_anon_4
                branch_sample_type
                sample_regs_user
                sample_stack_user
                #[cfg(feature = "linux-4.1")]
                clockid
                #[cfg(feature = "linux-3.19")]
                sample_regs_intr
                #[cfg(feature = "linux-4.1")]
                aux_watermark
                #[cfg(feature = "linux-4.8")]
                sample_max_stack
                #[cfg(feature = "linux-5.5")]
                aux_sample_size
                #[cfg(feature = "linux-5.13")]
                sig_data
                #[cfg(feature = "linux-6.3")]
                config3
        }

        f.write_str(" : ")?;

        debug_struct_fn! {
            name: __bindgen_anon_1
            self: self
            fmt: f
            fields:
                disabled
                inherit
                pinned
                exclusive

                exclude_user
                exclude_kernel
                exclude_hv
                exclude_idle

                mmap
                comm
                freq
                inherit_stat
                enable_on_exec
                task
                watermark
                precise_ip
                mmap_data
                sample_id_all

                exclude_host
                exclude_guest
                exclude_callchain_kernel
                exclude_callchain_user

                #[cfg(feature = "linux-3.12")]
                mmap2
                #[cfg(feature = "linux-3.16")]
                comm_exec
                #[cfg(feature = "linux-4.1")]
                use_clockid
                #[cfg(feature = "linux-4.3")]
                context_switch
                #[cfg(feature = "linux-4.7")]
                write_backward
                #[cfg(feature = "linux-4.12")]
                namespaces
                #[cfg(feature = "linux-5.1")]
                ksymbol
                #[cfg(feature = "linux-5.1")]
                bpf_event
                #[cfg(feature = "linux-5.4")]
                aux_output
                #[cfg(feature = "linux-5.7")]
                cgroup
                #[cfg(feature = "linux-5.9")]
                text_poke
                #[cfg(feature = "linux-5.12")]
                build_id
                #[cfg(feature = "linux-5.13")]
                inherit_thread
                #[cfg(feature = "linux-5.13")]
                remove_on_exec
                #[cfg(feature = "linux-5.13")]
                sigtrap
        }

        Ok(())
    }
}
