// Copyright (c) 2023-2025 Optimatist Technology Co., Ltd. All rights reserved.
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

use crate::syscall::bindings::*;
use std::ops::Not;

/// Sample ID structure used by all side-band records
///
/// This structure contains optional fields that can be included in
/// perf_event records.
#[derive(Debug, Clone)]
pub struct SampleId {
    pub pid: Option<u32>,
    pub tid: Option<u32>,
    pub time: Option<u64>,
    pub id_1: Option<u64>,
    pub stream_id: Option<u64>,
    pub cpu: Option<u32>,
    #[cfg(feature = "linux-3.12")]
    pub id_2: Option<u64>,
}

impl SampleId {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64) -> Self {
        let mut raw = SampleIdRaw {
            read_ptr: ptr,
            sample_type,
        };

        Self {
            pid: raw.pid().cloned(),
            tid: raw.tid().cloned(),
            time: raw.time().cloned(),
            id_1: raw.id_1().cloned(),
            stream_id: raw.stream_id().cloned(),
            cpu: raw.cpu().cloned(),
            #[cfg(feature = "linux-3.12")]
            id_2: raw.id_2().cloned(),
        }
    }
}

// Raw structure for parsing SampleId from memory
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SampleIdRaw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

type SampleIdMask = perf_event_sample_format;

macro_rules! gen_sample_id_fn {
    ($ty:ty, $name:ident $mask:expr) => {
        #[inline]
        pub unsafe fn $name(&mut self) -> Option<&$ty> {
            self.get_if($mask)
        }
    };
}

impl SampleIdRaw {
    #[inline]
    #[allow(clippy::unnecessary_cast)] // mask may be u64 or u32 in different linux headers
    const fn is_enabled(&self, mask: SampleIdMask) -> bool {
        (self.sample_type & mask as u64) > 0
    }

    #[inline]
    unsafe fn get_if<T>(&mut self, mask: SampleIdMask) -> Option<&T> {
        if self.is_enabled(mask).not() {
            return None;
        }
        let ptr = self.read_ptr as *const T;
        self.read_ptr = self.read_ptr.add(std::mem::size_of::<T>());
        ptr.as_ref()
    }

    gen_sample_id_fn! { u32, pid       PERF_SAMPLE_TID       }
    gen_sample_id_fn! { u32, tid       PERF_SAMPLE_TID       }
    gen_sample_id_fn! { u64, time      PERF_SAMPLE_TIME      }
    gen_sample_id_fn! { u64, id_1      PERF_SAMPLE_ID        }
    gen_sample_id_fn! { u64, stream_id PERF_SAMPLE_STREAM_ID }

    pub unsafe fn cpu(&mut self) -> Option<&u32> {
        if self.is_enabled(PERF_SAMPLE_CPU).not() {
            return None;
        }

        let cpu_ptr = self.read_ptr as *const u32;
        self.read_ptr = cpu_ptr.add(2) as _; // skip 32-bit res
        cpu_ptr.as_ref()
    }

    #[cfg(feature = "linux-3.12")]
    gen_sample_id_fn! { u64, id_2 PERF_SAMPLE_IDENTIFIER }
}

/// Base trait for flags that can be checked
pub trait FlagsTrait {
    fn raw(&self) -> u16;
    fn cpu_mode(&self) -> CpuMode;
}

/// Represents the misc field flags in perf_event_header
///
/// The misc field is a bitmask with some bits being reused for different
/// record types. This struct provides safe access to the misc based on
/// the record type context.
///
/// As the purpose of misc is represent flags of the record, we abstruct
/// the BasicFlags struct to represent the basic flags of the record.
#[derive(Debug, Clone, Copy)]
pub struct BasicFlags {
    raw: u16,
}

impl BasicFlags {
    pub fn new(raw: u16) -> Self {
        Self { raw }
    }

    /// Get the raw u16 value
    pub fn raw(&self) -> u16 {
        self.raw
    }

    /// Get CPU mode (mutually exclusive)
    pub fn cpu_mode(&self) -> CpuMode {
        let mode = self.raw & PERF_RECORD_MISC_CPUMODE_MASK as u16;
        match mode {
            val if val == PERF_RECORD_MISC_CPUMODE_UNKNOWN as u16 => CpuMode::Unknown,
            val if val == PERF_RECORD_MISC_KERNEL as u16 => CpuMode::Kernel,
            val if val == PERF_RECORD_MISC_USER as u16 => CpuMode::User,
            val if val == PERF_RECORD_MISC_HYPERVISOR as u16 => CpuMode::Hypervisor,
            val if val == PERF_RECORD_MISC_GUEST_KERNEL as u16 => CpuMode::GuestKernel,
            val if val == PERF_RECORD_MISC_GUEST_USER as u16 => CpuMode::GuestUser,
            _ => CpuMode::Unknown,
        }
    }
}

impl FlagsTrait for BasicFlags {
    fn raw(&self) -> u16 {
        self.raw()
    }

    fn cpu_mode(&self) -> CpuMode {
        self.cpu_mode()
    }
}

// Type-specific misc flags

/// Flags for MMAP records
#[derive(Debug, Clone, Copy)]
pub struct MmapFlags(BasicFlags);

impl MmapFlags {
    pub fn new(raw: u16) -> Self {
        Self(BasicFlags::new(raw))
    }

    /// For MMAP records: check if mapping is not executable
    pub fn is_mmap_data(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_MMAP_DATA as u16 != 0
    }
}

impl FlagsTrait for MmapFlags {
    fn raw(&self) -> u16 {
        self.0.raw()
    }

    fn cpu_mode(&self) -> CpuMode {
        self.0.cpu_mode()
    }
}

/// Flags for COMM records
#[derive(Debug, Clone, Copy)]
pub struct CommFlags(BasicFlags);

impl CommFlags {
    pub fn new(raw: u16) -> Self {
        Self(BasicFlags::new(raw))
    }

    /// For COMM records: check if name change was caused by execve(2)
    pub fn is_comm_exec(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_COMM_EXEC as u16 != 0
    }
}

impl FlagsTrait for CommFlags {
    fn raw(&self) -> u16 {
        self.0.raw()
    }

    fn cpu_mode(&self) -> CpuMode {
        self.0.cpu_mode()
    }
}

/// Flags for SWITCH records
#[derive(Debug, Clone, Copy)]
pub struct SwitchFlags(BasicFlags);

impl SwitchFlags {
    pub fn new(raw: u16) -> Self {
        Self(BasicFlags::new(raw))
    }

    /// For SWITCH records: check if context switch is away from current process
    pub fn is_switch_out(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_SWITCH_OUT as u16 != 0
    }

    /// For SWITCH records: check if context switch was a preemption
    pub fn is_switch_out_preempt(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_SWITCH_OUT_PREEMPT as u16 != 0
    }
}

impl FlagsTrait for SwitchFlags {
    fn raw(&self) -> u16 {
        self.0.raw()
    }

    fn cpu_mode(&self) -> CpuMode {
        self.0.cpu_mode()
    }
}

/// Flags for MMAP2 records
#[derive(Debug, Clone, Copy)]
pub struct Mmap2Flags(BasicFlags);

impl Mmap2Flags {
    pub fn new(raw: u16) -> Self {
        Self(BasicFlags::new(raw))
    }

    /// For MMAP records: check if mapping is not executable
    pub fn is_mmap_data(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_MMAP_DATA as u16 != 0
    }

    /// For MMAP2 records: check if build-ID data is present
    pub fn is_mmap_build_id(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_MMAP_BUILD_ID as u16 != 0
    }

    /// For MMAP2 records: check if /proc/pid/maps parsing was taking too long and was stopped
    /// This is a user-space perf utility flag, not set by the kernel
    pub fn is_proc_map_parse_timeout(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT as u16 != 0
    }
}

impl FlagsTrait for Mmap2Flags {
    fn raw(&self) -> u16 {
        self.0.raw()
    }

    fn cpu_mode(&self) -> CpuMode {
        self.0.cpu_mode()
    }
}

/// Flags for SAMPLE records
#[derive(Debug, Clone, Copy)]
pub struct SampleFlags(BasicFlags);

impl SampleFlags {
    pub fn new(raw: u16) -> Self {
        Self(BasicFlags::new(raw))
    }

    /// For SAMPLE records: check if IP points to the actual instruction that triggered the event
    pub fn is_exact_ip(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_EXACT_IP as u16 != 0
    }
}

impl FlagsTrait for SampleFlags {
    fn raw(&self) -> u16 {
        self.0.raw()
    }

    fn cpu_mode(&self) -> CpuMode {
        self.0.cpu_mode()
    }
}

/// Flags for FORK records
#[derive(Debug, Clone, Copy)]
pub struct ForkFlags(BasicFlags);

impl ForkFlags {
    pub fn new(raw: u16) -> Self {
        Self(BasicFlags::new(raw))
    }

    /// For FORK records: check if fork was caused by execve(2)
    pub fn is_fork_exec(&self) -> bool {
        self.0.raw & PERF_RECORD_MISC_FORK_EXEC as u16 != 0
    }
}

impl FlagsTrait for ForkFlags {
    fn raw(&self) -> u16 {
        self.0.raw()
    }

    fn cpu_mode(&self) -> CpuMode {
        self.0.cpu_mode()
    }
}

// Implement conversion traits for backward compatibility
impl From<u16> for BasicFlags {
    fn from(raw: u16) -> Self {
        Self::new(raw)
    }
}

impl Into<u16> for BasicFlags {
    fn into(self) -> u16 {
        self.raw()
    }
}

impl AsRef<u16> for BasicFlags {
    fn as_ref(&self) -> &u16 {
        &self.raw
    }
}

impl AsMut<u16> for BasicFlags {
    fn as_mut(&mut self) -> &mut u16 {
        &mut self.raw
    }
}

/// CPU mode enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CpuMode {
    Unknown,
    Kernel,
    User,
    Hypervisor,
    GuestKernel,
    GuestUser,
}

// Module declarations
#[cfg(feature = "linux-4.1")]
pub mod aux;
pub mod aux_output_hw_id;
#[cfg(feature = "linux-5.1")]
pub mod bpf_event;
#[cfg(feature = "linux-5.7")]
pub mod cgroup;
pub mod comm;
pub mod exit;
pub mod fork;
#[cfg(feature = "linux-4.1")]
pub mod itrace_start;
#[cfg(feature = "linux-5.1")]
pub mod ksymbol;
pub mod lost;
#[cfg(feature = "linux-4.2")]
pub mod lost_samples;
pub mod mmap;
#[cfg(feature = "linux-3.12")]
pub mod mmap2;
#[cfg(feature = "linux-4.12")]
pub mod namespaces;
pub mod read;
pub mod sample;
#[cfg(feature = "linux-4.3")]
pub mod switch;
#[cfg(feature = "linux-4.3")]
pub mod switch_cpu_wide;
#[cfg(feature = "linux-5.9")]
pub mod text_poke;
pub mod throttle;
pub mod unthrottle;

// Main Record enum with type-safe variants
#[derive(Debug, Clone)]
pub enum Record {
    #[cfg(feature = "linux-4.1")]
    Aux(AuxRecord),
    AuxOutputHwId(AuxOutputHwIdRecord),
    #[cfg(feature = "linux-4.1")]
    ItraceStart(ItraceStartRecord),
    #[cfg(feature = "linux-5.1")]
    BpfEvent(BpfEventRecord),
    #[cfg(feature = "linux-5.7")]
    Cgroup(CgroupRecord),
    Comm(CommRecord),
    Exit(ExitRecord),
    Fork(ForkRecord),
    #[cfg(feature = "linux-5.1")]
    Ksymbol(KsymbolRecord),
    Lost(LostRecord),
    #[cfg(feature = "linux-4.2")]
    LostSamples(LostSamplesRecord),
    Mmap(MmapRecord),
    #[cfg(feature = "linux-3.12")]
    Mmap2(Mmap2Record),
    #[cfg(feature = "linux-4.12")]
    Namespaces(NamespacesRecord),
    Read(ReadRecord),
    Sample(SampleRecord),
    #[cfg(feature = "linux-4.3")]
    Switch(SwitchRecord),
    #[cfg(feature = "linux-4.3")]
    SwitchCpuWide(SwitchCpuWideRecord),
    Throttle(ThrottleRecord),
    Unthrottle(UnthrottleRecord),
    #[cfg(feature = "linux-5.9")]
    TextPoke(TextPokeRecord),
}

// Re-export record types
#[cfg(feature = "linux-4.1")]
pub use aux::AuxRecord;
pub use aux_output_hw_id::AuxOutputHwIdRecord;
#[cfg(feature = "linux-5.1")]
pub use bpf_event::BpfEventRecord;
#[cfg(feature = "linux-5.7")]
pub use cgroup::CgroupRecord;
pub use comm::CommRecord;
pub use exit::ExitRecord;
pub use fork::ForkRecord;
#[cfg(feature = "linux-4.1")]
pub use itrace_start::ItraceStartRecord;
#[cfg(feature = "linux-5.1")]
pub use ksymbol::KsymbolRecord;
pub use lost::LostRecord;
#[cfg(feature = "linux-4.2")]
pub use lost_samples::LostSamplesRecord;
pub use mmap::MmapRecord;
#[cfg(feature = "linux-3.12")]
pub use mmap2::Mmap2Record;
#[cfg(feature = "linux-4.12")]
pub use namespaces::NamespacesRecord;
pub use read::ReadRecord;
pub use sample::SampleRecord;
pub use switch::SwitchRecord;
pub use switch_cpu_wide::SwitchCpuWideRecord;
#[cfg(feature = "linux-5.9")]
pub use text_poke::TextPokeRecord;
pub use throttle::ThrottleRecord;
pub use unthrottle::UnthrottleRecord;
