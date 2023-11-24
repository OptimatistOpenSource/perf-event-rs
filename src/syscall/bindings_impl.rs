use crate::syscall::bindings::*;
use std::fmt::{Debug, Formatter, Write};

macro_rules! debug_union {
    (
        self: $self:ident
        fmt: $f:ident
        struct: $struct:ident
        fields: $($field:ident)+
    ) => {
        $f.debug_struct(stringify!($struct))
            $(.field(stringify!($field), &unsafe { $self.$field }))+
            .finish()?;
    };
}

impl Debug for perf_event_attr__bindgen_ty_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            self: self
            fmt: f
            struct: perf_event_attr__bindgen_ty_1
            fields:
                sample_period
                sample_freq
        }

        Ok(())
    }
}

impl Debug for perf_event_attr__bindgen_ty_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            self: self
            fmt: f
            struct: perf_event_attr__bindgen_ty_2
            fields:
                wakeup_events
                wakeup_watermark
        }

        Ok(())
    }
}

impl Debug for perf_event_attr__bindgen_ty_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            self: self
            fmt: f
            struct: perf_event_attr__bindgen_ty_3
            fields:
                bp_addr
                kprobe_func
                uprobe_path
                config1
        }

        Ok(())
    }
}

impl Debug for perf_event_attr__bindgen_ty_4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            self: self
            fmt: f
            struct: perf_event_attr__bindgen_ty_4
            fields:
                bp_len
                kprobe_addr
                probe_offset
                config2
        }

        Ok(())
    }
}

macro_rules! debug_struct {
    (
        self: $self:ident
        fmt: $f:ident
        struct: $struct:ident
        fields: $($field:ident)+
    ) => {
        $f.debug_struct(stringify!($struct))
            $(.field(stringify!($field), &$self.$field))+
            .finish()?;
    };
}

impl Debug for perf_event_attr {
    // TODO: kernel version features
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_struct! {
            self: self
            fmt: f
            struct: perf_event_attr
            fields:
                type_
                size
                __bindgen_anon_1
                sample_type
                read_format
                __bindgen_anon_2
                bp_type
                __bindgen_anon_3
                __bindgen_anon_4
                branch_sample_type
                sample_regs_user
                sample_stack_user
                clockid
                sample_regs_intr
                aux_watermark
                sample_max_stack
                aux_sample_size
                sig_data
                config3
        }

        macro_rules! config_bit {
            ($fn_name:ident) => {{
                let val = self.$fn_name();
                f.write_fmt(format_args!("    {}: {}\n", stringify!($fn_name), val))?;
            }};
        }

        config_bit!(disabled);
        config_bit!(inherit);
        config_bit!(pinned);
        config_bit!(exclusive);

        config_bit!(exclude_user);
        config_bit!(exclude_kernel);
        config_bit!(exclude_hv);
        config_bit!(exclude_idle);

        config_bit!(mmap);
        config_bit!(comm);
        config_bit!(freq);
        config_bit!(inherit_stat);
        config_bit!(enable_on_exec);
        config_bit!(task);
        config_bit!(watermark);
        config_bit!(precise_ip);
        config_bit!(mmap_data);
        config_bit!(sample_id_all);

        config_bit!(exclude_host);
        config_bit!(exclude_guest);
        config_bit!(exclude_callchain_kernel);
        config_bit!(exclude_callchain_user);

        config_bit!(mmap2);
        config_bit!(comm_exec);
        config_bit!(use_clockid);
        config_bit!(context_switch);
        config_bit!(write_backward);
        config_bit!(namespaces);
        config_bit!(ksymbol);
        config_bit!(bpf_event);
        #[cfg(feature = "kernel-5.4")]
        config_bit!(aux_output);
        #[cfg(feature = "kernel-5.7")]
        config_bit!(cgroup);
        #[cfg(feature = "kernel-5.8")]
        config_bit!(text_poke);
        #[cfg(feature = "kernel-5.12")]
        config_bit!(build_id);
        #[cfg(feature = "kernel-5.13")]
        config_bit!(inherit_thread);
        #[cfg(feature = "kernel-5.13")]
        config_bit!(remove_on_exec);
        #[cfg(feature = "kernel-5.13")]
        config_bit!(sigtrap);

        Ok(())
    }
}
