use crate::counting::{Config, ExtraConfig};
use crate::infra::SizedExt;
use crate::perf_event::RawAttr;
use crate::syscall::bindings::*;
use crate::{Event, EventScope};

pub fn new(
    event: impl Into<Event>,
    scopes: impl IntoIterator<Item = EventScope>,
    extra_config: ExtraConfig,
) -> Config {
    let mut raw_attr = RawAttr {
        type_: 0,
        size: RawAttr::size() as _,
        config: 0,
        __bindgen_anon_1: perf_event_attr__bindgen_ty_1::default(), // not use in counting mode
        sample_type: 0,                                             // ditto
        read_format: {
            #[allow(unused_mut)]
            #[allow(clippy::identity_op)] // for readable
            let mut read_format = 0
                | PERF_FORMAT_TOTAL_TIME_ENABLED
                | PERF_FORMAT_TOTAL_TIME_RUNNING
                | PERF_FORMAT_ID
                | PERF_FORMAT_GROUP;

            // WARN: set for read_format align, event_lost should not be used.
            #[cfg(feature = "linux-6.0")]
            {
                read_format |= PERF_FORMAT_LOST;
            }

            read_format
        } as _,
        _bitfield_align_1: [],
        _bitfield_1: __BindgenBitfieldUnit::new([0u8; 8usize]), // set latter via raw_attr.set_...
        __bindgen_anon_2: perf_event_attr__bindgen_ty_2::default(), // not use in counting mode
        bp_type: 0,                                             // ditto
        __bindgen_anon_3: perf_event_attr__bindgen_ty_3::default(), // ditto
        __bindgen_anon_4: perf_event_attr__bindgen_ty_4::default(), // ditto
        // TODO: config1 in __bindgen_anon_3
        // TODO: config2 in __bindgen_anon_4
        branch_sample_type: 0, // ditto
        sample_regs_user: 0,   // ditto
        sample_stack_user: 0,  // ditto
        clockid: 0,            // ditto
        sample_regs_intr: 0,   // ditto
        aux_watermark: 0,      // ditto
        sample_max_stack: 0,   // ditto
        __reserved_2: 0,
        #[cfg(feature = "linux-5.5")]
        aux_sample_size: 0, // not use in counting mode
        __reserved_3: 0,
        #[cfg(feature = "linux-5.13")]
        sig_data: 0, // not use in counting mode

        // TODO: https://github.com/torvalds/linux/commit/09519ec3b19e4144b5f6e269c54fbb9c294a9fcb
        #[cfg(feature = "linux-6.3")]
        config3: 0,
    };

    raw_attr.set_disabled(1);
    raw_attr.set_inherit(extra_config.inherit as _);
    raw_attr.set_pinned(extra_config.pinned as _);
    raw_attr.set_exclusive(extra_config.exclusive as _);

    raw_attr.set_exclude_user(1);
    raw_attr.set_exclude_kernel(1);
    raw_attr.set_exclude_hv(1);
    raw_attr.set_exclude_idle(1);

    raw_attr.set_mmap(0); // not use in counting mode
    raw_attr.set_comm(0); // ditto
    raw_attr.set_freq(0); // ditto
    raw_attr.set_inherit_stat(extra_config.inherit_stat as _);
    raw_attr.set_enable_on_exec(extra_config.enable_on_exec as _);
    raw_attr.set_task(0); // not use in counting mode
    raw_attr.set_watermark(0); // ditto
    raw_attr.set_precise_ip(0); // ditto
    raw_attr.set_mmap_data(0); // ditto
    raw_attr.set_sample_id_all(0); // ditto

    raw_attr.set_exclude_host(1);
    raw_attr.set_exclude_guest(1);
    raw_attr.set_exclude_callchain_kernel(1);
    raw_attr.set_exclude_callchain_user(1);

    raw_attr.set_mmap2(0); // not use in counting mode
    raw_attr.set_comm_exec(0); // ditto
    raw_attr.set_use_clockid(0); // ditto
    raw_attr.set_context_switch(0); // ditto
    raw_attr.set_write_backward(0); // ditto
    raw_attr.set_namespaces(0); // ditto
    raw_attr.set_ksymbol(0); // ditto
    raw_attr.set_bpf_event(0); // ditto
    #[cfg(feature = "linux-5.4")]
    raw_attr.set_aux_output(0); // ditto
    #[cfg(feature = "linux-5.7")]
    raw_attr.set_cgroup(0); // ditto
    #[cfg(feature = "linux-5.8")]
    raw_attr.set_text_poke(0); // ditto
    #[cfg(feature = "linux-5.12")]
    raw_attr.set_build_id(0); // ditto
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_inherit_thread(extra_config.inherit_thread as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_remove_on_exec(extra_config.remove_on_exec as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_sigtrap(0); // not use in counting mode

    use EventScope::*;
    scopes.into_iter().for_each(|scope| match scope {
        User => raw_attr.set_exclude_user(0),
        Kernel => raw_attr.set_exclude_kernel(0),
        Hv => raw_attr.set_exclude_hv(0),
        Idle => raw_attr.set_exclude_idle(0),
        Host => raw_attr.set_exclude_host(0),
        Guest => raw_attr.set_exclude_guest(0),
        CallchainKernel => raw_attr.set_exclude_callchain_kernel(0),
        CallchainUser => raw_attr.set_exclude_callchain_user(0),
    });

    match event.into() {
        Event::Hardware(ev) if ev.is_cache_event() => {
            raw_attr.type_ = PERF_TYPE_HW_CACHE;
            raw_attr.config = ev.into_u64();
        }
        Event::Hardware(ev) => {
            raw_attr.type_ = PERF_TYPE_HARDWARE;
            raw_attr.config = ev.into_u64();
        }
        Event::Sw(ev) => {
            raw_attr.type_ = PERF_TYPE_SOFTWARE;
            raw_attr.config = ev.into_u64();
        }
        Event::Raw(ev) => {
            raw_attr.type_ = PERF_TYPE_RAW;
            raw_attr.config = ev.into_u64();
        }
    }

    Config { raw_attr }
}
