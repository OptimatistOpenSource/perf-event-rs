use crate::syscall::bindings::*;
use std::fmt::{Debug, Formatter};

macro_rules! debug_union {
    (
        name: $name:ident
        self: $self:ident
        fmt: $f:ident
        fields: $($(#[$attr:meta])* $field:ident)+
    ) => {{
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &unsafe { $self.$field });
        )+
        ds.finish()?;
    }};
}

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

impl Debug for perf_event_attr__bindgen_ty_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_union! {
            name: perf_event_attr__bindgen_ty_3
            self: self
            fmt: f
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

macro_rules! debug_struct {
    (
        name: $name:ident
        self: $self:ident
        fmt: $f:ident
        fields: $($(#[$attr:meta])* $field:ident)+
    ) => {
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &$self.$field);
        )+
        ds.finish()?;
    };
}

macro_rules! debug_bits {
    (
        name: $name:ident
        self: $self:ident
        fmt: $f:ident
        fields: $($(#[$attr:meta])* $field:ident)+
    ) => {
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &$self.$field());
        )+
        ds.finish()?;
    };
}

impl Debug for perf_event_attr {
    // TODO: kernel version features
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
                clockid
                sample_regs_intr
                aux_watermark
                sample_max_stack
                aux_sample_size
                sig_data
                config3
        }

        f.write_str(" : ")?;

        debug_bits! {
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

                mmap2
                comm_exec
                use_clockid
                context_switch
                write_backward
                namespaces
                ksymbol
                bpf_event
                #[cfg(feature = "kernel-5.4")]
                aux_output
                #[cfg(feature = "kernel-5.7")]
                cgroup
                #[cfg(feature = "kernel-5.8")]
                text_poke
                #[cfg(feature = "kernel-5.12")]
                build_id
                #[cfg(feature = "kernel-5.13")]
                inherit_thread
                #[cfg(feature = "kernel-5.13")]
                remove_on_exec
                #[cfg(feature = "kernel-5.13")]
                sigtrap
        }

        Ok(())
    }
}
