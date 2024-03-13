// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License 
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// 
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not, 
// see <https://www.gnu.org/licenses/>.

use crate::perf_event::RawAttr;
#[cfg(feature = "linux-4.1")]
use crate::sampling::ClockId;
use crate::sampling::{Config, ExtraConfig, OverflowBy, SampleIpSkid, Wakeup};
use crate::syscall::bindings::*;
#[cfg(feature = "linux-4.17")]
use crate::{DynamicPmuEvent, KprobeConfig, UprobeConfig};
use crate::{Event, EventScope};
#[cfg(feature = "linux-4.1")]
use libc::{CLOCK_BOOTTIME, CLOCK_MONOTONIC, CLOCK_MONOTONIC_RAW, CLOCK_REALTIME, CLOCK_TAI};
use std::mem::size_of;
use std::ops::Not;

#[inline]
pub fn new<'t>(
    event: &Event,
    scopes: impl IntoIterator<Item = &'t EventScope>,
    overflow_by: &OverflowBy,
    extra_config: &ExtraConfig,
) -> Config {
    let sample_record_fields = &extra_config.sample_record_fields;

    let mut raw_attr = RawAttr {
        type_: 0,
        size: size_of::<RawAttr>() as _,
        config: 0,
        __bindgen_anon_1: match overflow_by {
            OverflowBy::Freq(f) => perf_event_attr__bindgen_ty_1 { sample_freq: *f },
            OverflowBy::Period(p) => perf_event_attr__bindgen_ty_1 { sample_period: *p },
        },
        sample_type: sample_record_fields.as_sample_type(),
        read_format: {
            #[allow(unused_mut)]
            #[allow(clippy::identity_op)] // for readable
            let mut val = 0
                | PERF_FORMAT_TOTAL_TIME_ENABLED
                | PERF_FORMAT_TOTAL_TIME_RUNNING
                | PERF_FORMAT_ID
                | PERF_FORMAT_GROUP;

            #[cfg(feature = "linux-6.0")]
            {
                val |= PERF_FORMAT_LOST;
            }

            val as _
        },
        _bitfield_align_1: [],
        // set later via raw_attr.set_...
        _bitfield_1: __BindgenBitfieldUnit::new([0u8; 8usize]),
        __bindgen_anon_2: match &extra_config.wakeup {
            Wakeup::Events(val) => perf_event_attr__bindgen_ty_2 {
                wakeup_events: *val,
            },
            Wakeup::Watermark(val) => perf_event_attr__bindgen_ty_2 {
                wakeup_watermark: *val,
            },
        },

        // The following 3 items are later set through event.enable_in_raw_attr...
        bp_type: 0,
        __bindgen_anon_3: perf_event_attr__bindgen_ty_3::default(),
        __bindgen_anon_4: perf_event_attr__bindgen_ty_4::default(),

        branch_sample_type: 0, // TODO: Not all hardware supports this feature
        sample_regs_user: sample_record_fields.abi_and_regs_user.unwrap_or(0),
        sample_stack_user: sample_record_fields.data_stack_user.unwrap_or(0) as _,
        #[rustfmt::skip]
        #[cfg(feature = "linux-4.1")]
        clockid: extra_config.clockid.as_ref().map_or(0, |id| match id {
            ClockId::Monotonic    => CLOCK_MONOTONIC,
            ClockId::MonotonicRaw => CLOCK_MONOTONIC_RAW,
            ClockId::RealTime     => CLOCK_REALTIME,
            ClockId::BootTime     => CLOCK_BOOTTIME,
            ClockId::Tai          => CLOCK_TAI,
        }) as _,
        #[cfg(feature = "linux-3.19")]
        sample_regs_intr: sample_record_fields.abi_and_regs_intr.unwrap_or(0),
        #[cfg(feature = "linux-4.1")]
        aux_watermark: 0, // TODO
        #[cfg(feature = "linux-4.8")]
        sample_max_stack: sample_record_fields.ips.unwrap_or(0),
        __reserved_2: 0,
        #[cfg(feature = "linux-5.5")]
        aux_sample_size: 0, // TODO
        #[cfg(feature = "linux-5.5")]
        __reserved_3: 0,
        #[cfg(feature = "linux-5.13")]
        sig_data: extra_config.sigtrap.unwrap_or(0),

        // TODO: https://github.com/torvalds/linux/commit/09519ec3b19e4144b5f6e269c54fbb9c294a9fcb
        #[cfg(feature = "linux-6.3")]
        config3: 0,
    };

    raw_attr.set_disabled(1);

    /*
    Line 6402 of kernel/events/core.c:
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

    raw_attr.set_exclude_callchain_kernel(extra_config.include_callchain_kernel.not() as _);
    raw_attr.set_exclude_callchain_user(extra_config.include_callchain_user.not() as _);

    #[cfg(feature = "linux-3.12")]
    raw_attr.set_mmap2(0);
    #[cfg(feature = "linux-3.16")]
    raw_attr.set_comm_exec(extra_config.comm_exec as _);
    #[cfg(feature = "linux-4.1")]
    raw_attr.set_use_clockid(extra_config.clockid.is_some() as _);
    #[cfg(feature = "linux-4.3")]
    raw_attr.set_context_switch(0);
    #[cfg(feature = "linux-4.7")]
    raw_attr.set_write_backward(0);
    #[cfg(feature = "linux-4.12")]
    raw_attr.set_namespaces(0);
    #[cfg(feature = "linux-5.1")]
    raw_attr.set_ksymbol(0);
    #[cfg(feature = "linux-5.1")]
    raw_attr.set_bpf_event(0);
    #[cfg(feature = "linux-5.4")]
    //raw_attr.set_aux_output(extra_config.aux_output as _);
    raw_attr.set_aux_output(0);
    #[cfg(feature = "linux-5.7")]
    raw_attr.set_cgroup(0);
    #[cfg(feature = "linux-5.9")]
    raw_attr.set_text_poke(0);
    #[cfg(feature = "linux-5.12")]
    raw_attr.set_build_id(extra_config.build_id as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_inherit_thread(extra_config.inherit_thread as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_remove_on_exec(extra_config.remove_on_exec as _);
    #[cfg(feature = "linux-5.13")]
    raw_attr.set_sigtrap(extra_config.sigtrap.is_some() as _);

    event.enable_in_raw_attr(&mut raw_attr);

    scopes
        .into_iter()
        .for_each(|scope| scope.enable_in_raw_attr(&mut raw_attr));

    extra_config
        .extra_record_types
        .iter()
        .for_each(|it| it.enable_in_raw_attr(&mut raw_attr));

    let kprobe_func_or_uprobe_path = match event {
        #[cfg(feature = "linux-4.17")]
        Event::DynamicPmu(DynamicPmuEvent::Kprobe {
            cfg: KprobeConfig::FuncAndOffset { kprobe_func, .. },
            ..
        }) => Some(kprobe_func.clone()),
        #[cfg(feature = "linux-4.17")]
        Event::DynamicPmu(DynamicPmuEvent::Uprobe {
            cfg: UprobeConfig { uprobe_path, .. },
            ..
        }) => Some(uprobe_path.clone()),
        _ => None,
    };

    Config {
        kprobe_func_or_uprobe_path,
        raw_attr,
    }
}
