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

mod abi_and_regs;
mod data_src;
mod raw;
mod weight;

use crate::sampling::SamplerGroupStat;
use crate::syscall::bindings::*;
pub use abi_and_regs::*;
pub use data_src::*;
pub use weight::*;

#[derive(Debug, Clone)]
pub struct Body {
    #[cfg(feature = "linux-3.12")]
    pub sample_id: Option<u64>,
    pub ip: Option<u64>,
    pub pid: Option<u32>,
    pub tid: Option<u32>,
    pub time: Option<u64>,
    pub addr: Option<u64>,
    pub id: Option<u64>,
    pub stream_id: Option<u64>,
    pub cpu: Option<u32>,
    pub period: Option<u64>,
    pub v: Option<SamplerGroupStat>,
    pub ips: Option<Vec<u64>>,
    pub data_raw: Option<Vec<u8>>,
    pub abi_and_regs_user: Option<AbiAndRegs>,
    pub data_stack_user: Option<Vec<u8>>,
    pub weight: Option<Weight>,
    pub data_src: Option<DataSrc>,
    #[cfg(feature = "linux-3.13")]
    pub transaction: Option<u64>,
    #[cfg(feature = "linux-3.19")]
    pub abi_and_regs_intr: Option<AbiAndRegs>,
    #[cfg(feature = "linux-4.14")]
    pub phys_addr: Option<u64>,
    #[cfg(feature = "linux-5.7")]
    pub cgroup: Option<u64>,
    #[cfg(feature = "linux-5.11")]
    pub data_page_size: Option<u64>,
    #[cfg(feature = "linux-5.11")]
    pub code_page_size: Option<u64>,
}

impl Body {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        sample_type: u64,
        regs_user_len: usize,
        #[cfg(feature = "linux-3.19")] regs_intr_len: usize,
    ) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        Self {
            #[cfg(feature = "linux-3.12")]
            sample_id: raw.sample_id().cloned(),
            ip: raw.ip().cloned(),
            pid: raw.pid().cloned(),
            tid: raw.tid().cloned(),
            time: raw.time().cloned(),
            addr: raw.addr().cloned(),
            id: raw.id().cloned(),
            stream_id: raw.stream_id().cloned(),
            cpu: raw.cpu().cloned(),
            period: raw.period().cloned(),
            v: raw.v().map(|(h, b)| SamplerGroupStat::from_raw(h, b)),
            ips: raw.ips().map(|it| it.to_vec()),
            data_raw: raw.data_raw().map(|it| it.to_vec()),
            abi_and_regs_user: raw
                .abi_and_regs_user(regs_user_len)
                .map(AbiAndRegs::from_raw),
            data_stack_user: raw.data_stack_user().map(|it| it.to_vec()),
            weight: raw.weight().map(|it| {
                let repr = match sample_type {
                    // mask may be u64 or u32 in different linux headers
                    #[allow(clippy::unnecessary_cast)]
                    st if (st & PERF_SAMPLE_WEIGHT as u64) > 0 => WeightRepr::Full,
                    #[cfg(feature = "linux-5.12")]
                    // mask may be u64 or u32 in different linux headers
                    #[allow(clippy::unnecessary_cast)]
                    st if (st & PERF_SAMPLE_WEIGHT_STRUCT as u64) > 0 => WeightRepr::Vars,
                    _ => unreachable!(),
                };
                Weight::from_raw(*it, repr)
            }),
            data_src: raw.data_src().cloned().map(DataSrc::from_raw),
            #[cfg(feature = "linux-3.13")]
            transaction: raw.transaction().cloned(),
            #[cfg(feature = "linux-3.19")]
            abi_and_regs_intr: raw
                .abi_and_regs_intr(regs_intr_len)
                .map(AbiAndRegs::from_raw),
            #[cfg(feature = "linux-4.14")]
            phys_addr: raw.phys_addr().cloned(),
            #[cfg(feature = "linux-5.7")]
            cgroup: raw.cgroup().cloned(),
            #[cfg(feature = "linux-5.11")]
            data_page_size: raw.data_page_size().cloned(),
            #[cfg(feature = "linux-5.11")]
            code_page_size: raw.code_page_size().cloned(),
        }
    }
}
