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

use crate::sampling::config::sample_record_fields::SampleRecordFields;
use crate::sampling::ExtraRecord;
use crate::sampling::Wakeup::Events;

#[derive(Debug, Clone)]
pub struct ExtraConfig {
    pub pinned: bool,
    pub exclusive: bool,
    pub mmap_data: bool,

    pub comm: bool,
    #[cfg(feature = "linux-3.16")]
    pub comm_exec: bool,

    /// TODO: `inherit` can't be turned on when `sample_record_fields.v` is enabled
    pub inherit: bool,
    pub inherit_stat: bool,
    pub inherit_thread: bool,

    //#[cfg(feature = "linux-5.4")]
    //pub aux_output: bool,
    #[cfg(feature = "linux-5.12")]
    pub build_id: bool,

    pub enable_on_exec: bool,
    #[cfg(feature = "linux-5.13")]
    pub remove_on_exec: bool,

    pub include_callchain_kernel: bool,
    pub include_callchain_user: bool,

    #[cfg(feature = "linux-4.1")]
    pub clockid: Option<ClockId>,
    pub precise_ip: SampleIpSkid,
    pub wakeup: Wakeup,
    /// Wrap `sig_data` with `Some` to enable sigtrap
    pub sigtrap: Option<u64>,

    pub sample_record_fields: SampleRecordFields,

    pub extra_record_types: Vec<ExtraRecord>,
    /// i.e. `sample_id_all`
    pub extra_record_with_sample_id: bool,
}

impl Default for ExtraConfig {
    fn default() -> Self {
        Self {
            pinned: false,
            exclusive: false,
            mmap_data: false,

            comm: false,
            #[cfg(feature = "linux-3.16")]
            comm_exec: false,

            inherit: false,
            inherit_stat: false,
            inherit_thread: false,

            //#[cfg(feature = "linux-5.4")]
            //aux_output: false,
            #[cfg(feature = "linux-5.12")]
            build_id: false,

            enable_on_exec: false,
            #[cfg(feature = "linux-5.13")]
            remove_on_exec: false,

            include_callchain_kernel: false,
            include_callchain_user: false,

            #[cfg(feature = "linux-4.1")]
            clockid: None,
            precise_ip: SampleIpSkid::Arbitrary,
            wakeup: Events(1),
            sigtrap: None,

            sample_record_fields: Default::default(),

            extra_record_types: vec![],
            extra_record_with_sample_id: false,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SampleIpSkid {
    /// SAMPLE_IP can have arbitrary skid
    Arbitrary, // 0
    /// SAMPLE_IP must have constant skid.
    Constant, // 1
    /// SAMPLE_IP requested to have 0 skid.
    TryZero, // 2
    /// SAMPLE_IP must have 0 skid.  See also the
    /// description of PERF_RECORD_MISC_EXACT_IP.
    Zero, // 3
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ClockId {
    Monotonic,
    MonotonicRaw,
    RealTime,
    BootTime,
    Tai,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Wakeup {
    Events(u32),
    Watermark(u32),
}
