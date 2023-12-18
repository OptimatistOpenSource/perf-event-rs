use crate::infra::SizedExt;
use crate::perf_event::RawAttr;
use crate::sampling::{
    ClockId, Config, ExtraConfig, ExtraRecord, OverflowBy, SampleIpSkid, Wakeup,
};
use crate::syscall::bindings::*;
use crate::{Event, EventScope};
use libc::{CLOCK_BOOTTIME, CLOCK_MONOTONIC, CLOCK_MONOTONIC_RAW, CLOCK_REALTIME, CLOCK_TAI};
use std::ops::Not;

pub fn new(
    event: impl Into<Event>,
    scopes: impl IntoIterator<Item = EventScope>,
    overflow_by: OverflowBy,
    extra_config: &ExtraConfig,
) -> Config {
    let sample_record_fields = &extra_config.sample_record_fields;

    let mut raw_attr = RawAttr {
        type_: 0,
        size: RawAttr::size() as _,
        config: 0,
        __bindgen_anon_1: match overflow_by {
            OverflowBy::Freq(f) => perf_event_attr__bindgen_ty_1 { sample_freq: f },
            OverflowBy::Period(p) => perf_event_attr__bindgen_ty_1 { sample_period: p },
        },
        sample_type: sample_record_fields.as_sample_type(),
        read_format: {
            #[allow(unused_mut)]
            #[allow(clippy::identity_op)] // for readable
            let mut read_format = 0
                | PERF_FORMAT_TOTAL_TIME_ENABLED
                | PERF_FORMAT_TOTAL_TIME_RUNNING
                | PERF_FORMAT_ID
                | PERF_FORMAT_GROUP;

            #[cfg(feature = "linux-6.0")]
            {
                read_format |= PERF_FORMAT_LOST;
            }

            read_format
        } as _,
        _bitfield_align_1: [],
        _bitfield_1: __BindgenBitfieldUnit::new([0u8; 8usize]), // set latter via raw_attr.set_...
        __bindgen_anon_2: match &extra_config.wakeup {
            Wakeup::Events(val) => perf_event_attr__bindgen_ty_2 {
                wakeup_events: *val,
            },
            Wakeup::Watermark(val) => perf_event_attr__bindgen_ty_2 {
                wakeup_watermark: *val,
            },
        },
        bp_type: 0, // not use in sampling mode
        __bindgen_anon_3: perf_event_attr__bindgen_ty_3::default(), // ditto
        __bindgen_anon_4: perf_event_attr__bindgen_ty_4::default(), // ditto
        // TODO: config1 in __bindgen_anon_3
        // TODO: config2 in __bindgen_anon_4
        branch_sample_type: 0, // TODO: Not all hardware supports this feature
        sample_regs_user: sample_record_fields.abi_and_regs_user.unwrap_or(0),
        sample_stack_user: sample_record_fields.data_stack_user.unwrap_or(0) as _,
        clockid: extra_config.clockid.as_ref().map_or(0, |id| match id {
            ClockId::Monotonic => CLOCK_MONOTONIC,
            ClockId::MonotonicRaw => CLOCK_MONOTONIC_RAW,
            ClockId::RealTime => CLOCK_REALTIME,
            ClockId::BootTime => CLOCK_BOOTTIME,
            ClockId::Tai => CLOCK_TAI,
        }) as _,
        sample_regs_intr: sample_record_fields.abi_and_regs_intr.unwrap_or(0),
        aux_watermark: 0, // TODO
        sample_max_stack: sample_record_fields.ips.unwrap_or(0),
        __reserved_2: 0,
        #[cfg(feature = "linux-5.5")]
        aux_sample_size: 0, // TODO
        __reserved_3: 0,
        #[cfg(feature = "linux-5.13")]
        sig_data: extra_config.sigtrap.unwrap_or(0),
        #[cfg(feature = "linux-latest")]
        config3: 0, // TODO: missing docs in manual
    };

    raw_attr.set_disabled(1);

    /*
    From line 6402 of kernel/events/core.c:
    Don't allow mmap() of inherited per-task counters. This would
    create a performance issue due to all children writing to the
    same rb.
    */
    raw_attr.set_inherit(extra_config.inherit as _);
    raw_attr.set_pinned(extra_config.pinned as _);
    raw_attr.set_exclusive(extra_config.exclusive as _);

    raw_attr.set_exclude_user(1);
    raw_attr.set_exclude_kernel(1);
    raw_attr.set_exclude_hv(1);
    raw_attr.set_exclude_idle(1);

    raw_attr.set_mmap(0);
    raw_attr.set_comm(extra_config.comm as _);
    raw_attr.set_freq(match overflow_by {
        OverflowBy::Freq(_) => 1,
        OverflowBy::Period(_) => 0,
    });
    raw_attr.set_inherit_stat(extra_config.inherit_stat as _); // `inherit_stat` requires `inherit` to be enabled
    raw_attr.set_enable_on_exec(extra_config.enable_on_exec as _);
    raw_attr.set_task(0);
    raw_attr.set_watermark(match extra_config.wakeup {
        Wakeup::Watermark(_) => 1,
        Wakeup::Events(_) => 0,
    });
    raw_attr.set_precise_ip(match extra_config.precise_ip {
        SampleIpSkid::Arbitrary => 0,
        SampleIpSkid::Constant => 1,
        SampleIpSkid::TryZero => 2,
        SampleIpSkid::Zero => 3,
    });
    raw_attr.set_mmap_data(extra_config.mmap_data as _);

    if extra_config.extra_record_with_sample_id && extra_config.extra_record_types.is_empty().not()
    {
        raw_attr.set_sample_id_all(1);
    } else {
        raw_attr.set_sample_id_all(0);
    }

    raw_attr.set_exclude_host(1);
    raw_attr.set_exclude_guest(1);
    raw_attr.set_exclude_callchain_kernel(1);
    raw_attr.set_exclude_callchain_user(1);

    raw_attr.set_mmap2(0);
    raw_attr.set_comm_exec(extra_config.comm_exec as _);
    raw_attr.set_use_clockid(extra_config.clockid.is_some() as _);
    raw_attr.set_context_switch(0);
    raw_attr.set_write_backward(0);
    raw_attr.set_namespaces(0);
    raw_attr.set_ksymbol(0);
    raw_attr.set_bpf_event(0);
    #[cfg(feature = "linux-5.4")]
    //raw_attr.set_aux_output(extra_config.aux_output as _);
    raw_attr.set_aux_output(0);
    #[cfg(feature = "linux-5.7")]
    raw_attr.set_cgroup(0);
    #[cfg(feature = "linux-5.8")]
    raw_attr.set_text_poke(0);
    #[cfg(feature = "linux-5.12")]
    raw_attr.set_build_id(extra_config.build_id as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_inherit_thread(extra_config.inherit_thread as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_remove_on_exec(extra_config.remove_on_exec as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_sigtrap(extra_config.sigtrap.is_some() as _);

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
        Event::Hw(ev) if ev.is_cache_event() => {
            raw_attr.type_ = PERF_TYPE_HW_CACHE;
            raw_attr.config = ev.into_u64();
        }
        Event::Hw(ev) => {
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

    extra_config
        .extra_record_types
        .iter()
        .for_each(|it| match it {
            ExtraRecord::Mmap => raw_attr.set_mmap(1),
            ExtraRecord::Mmap2 => raw_attr.set_mmap2(1),
            ExtraRecord::ContextSwitch => raw_attr.set_context_switch(1),
            ExtraRecord::Namespaces => raw_attr.set_namespaces(1),
            ExtraRecord::Ksymbol => raw_attr.set_ksymbol(1),
            ExtraRecord::BpfEvent => raw_attr.set_bpf_event(1),
            #[cfg(feature = "linux-5.7")]
            ExtraRecord::Cgroup => raw_attr.set_cgroup(1),
            #[cfg(feature = "linux-5.8")]
            ExtraRecord::TextPoke => raw_attr.set_text_poke(1),
            ExtraRecord::ForkAndExit => raw_attr.set_task(1),
        });

    Config { raw_attr }
}
