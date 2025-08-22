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

use crate::sampling::record::SampleFlags;
use crate::sampling::SamplerGroupStat;
use crate::syscall::bindings::*;
pub use abi_and_regs::*;
pub use data_src::*;
use std::fmt;
pub use weight::*;

#[derive(Debug, Clone)]
pub struct SampleRecord {
    pub basic_flags: SampleFlags,
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
    pub read: Option<SamplerGroupStat>,
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

impl SampleRecord {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        sample_type: u64,
        regs_user_len: usize,
        #[cfg(feature = "linux-3.19")] regs_intr_len: usize,
        flags: u16,
    ) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        Self {
            basic_flags: SampleFlags::new(flags),
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
            read: raw.v().map(|(h, b)| SamplerGroupStat::from_raw(h, b)),
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

impl fmt::Display for SampleRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sample Record:")?;

        // Basic information
        if let Some(ip) = self.ip {
            writeln!(f, "  IP: 0x{:x}", ip)?;
        }
        if let Some(pid) = self.pid {
            writeln!(f, "  PID: {}", pid)?;
        }
        if let Some(tid) = self.tid {
            writeln!(f, "  TID: {}", tid)?;
        }
        if let Some(time) = self.time {
            writeln!(f, "  Time: {}", time)?;
        }
        if let Some(addr) = self.addr {
            writeln!(f, "  Addr: 0x{:x}", addr)?;
        }
        if let Some(cpu) = self.cpu {
            writeln!(f, "  CPU: {}", cpu)?;
        }
        if let Some(period) = self.period {
            writeln!(f, "  Period: {}", period)?;
        }

        // Sample ID and stream information
        #[cfg(feature = "linux-3.12")]
        if let Some(sample_id) = self.sample_id {
            writeln!(f, "  Sample ID: 0x{:x}", sample_id)?;
        }
        if let Some(id) = self.id {
            writeln!(f, "  ID: 0x{:x}", id)?;
        }
        if let Some(stream_id) = self.stream_id {
            writeln!(f, "  Stream ID: 0x{:x}", stream_id)?;
        }

        // Call chain information
        if let Some(ips) = &self.ips {
            writeln!(f, "  Call Chain ({} entries):", ips.len())?;
            for (i, ip) in ips.iter().enumerate() {
                writeln!(f, "    [{}] 0x{:x}", i, ip)?;
            }
        }

        // Raw data
        if let Some(data_raw) = &self.data_raw {
            writeln!(f, "  Raw Data ({} bytes):", data_raw.len())?;
            if data_raw.len() <= 32 {
                writeln!(f, "    {:02x?}", data_raw)?;
            } else {
                writeln!(f, "    {:02x?}...", &data_raw[..32])?;
            }
        }

        // User registers
        if let Some(regs) = &self.abi_and_regs_user {
            writeln!(f, "  User Registers:")?;
            writeln!(f, "    {:?}", regs)?;
        }

        // Interrupt registers
        #[cfg(feature = "linux-3.19")]
        if let Some(regs) = &self.abi_and_regs_intr {
            writeln!(f, "  Interrupt Registers:")?;
            writeln!(f, "    {:?}", regs)?;
        }

        // Stack data
        if let Some(stack) = &self.data_stack_user {
            writeln!(f, "  User Stack ({} bytes):", stack.len())?;
            if stack.len() <= 64 {
                writeln!(f, "    {:02x?}", stack)?;
            } else {
                writeln!(f, "    {:02x?}...", &stack[..64])?;
            }
        }

        // Weight information
        if let Some(weight) = &self.weight {
            writeln!(f, "  Weight: {:?}", weight)?;
        }

        // Data source
        if let Some(data_src) = &self.data_src {
            writeln!(f, "  Data Source: {:?}", data_src)?;
        }

        // Transaction
        #[cfg(feature = "linux-3.13")]
        if let Some(transaction) = self.transaction {
            writeln!(f, "  Transaction: 0x{:x}", transaction)?;
        }

        // Physical address
        #[cfg(feature = "linux-4.14")]
        if let Some(phys_addr) = self.phys_addr {
            writeln!(f, "  Physical Address: 0x{:x}", phys_addr)?;
        }

        // Cgroup
        #[cfg(feature = "linux-5.7")]
        if let Some(cgroup) = self.cgroup {
            writeln!(f, "  Cgroup: 0x{:x}", cgroup)?;
        }

        // Page sizes
        #[cfg(feature = "linux-5.11")]
        if let Some(data_page_size) = self.data_page_size {
            writeln!(f, "  Data Page Size: 0x{:x}", data_page_size)?;
        }
        #[cfg(feature = "linux-5.11")]
        if let Some(code_page_size) = self.code_page_size {
            writeln!(f, "  Code Page Size: 0x{:x}", code_page_size)?;
        }

        // Group statistics
        if let Some(v) = &self.read {
            writeln!(f, "  Group Stats: {:?}", v)?;
        }

        Ok(())
    }
}
