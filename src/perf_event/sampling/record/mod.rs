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

mod body;
mod sample_id;

pub use body::*;
pub use sample_id::*;

#[derive(Debug, Clone)]
pub struct Record {
    pub misc: u16,
    pub body: RecordBody,
}

#[derive(Debug, Clone)]
pub enum RecordBody {
    Mmap(Box<mmap::Body>),
    Lost(Box<lost::Body>),
    Comm(Box<comm::Body>),
    Exit(Box<exit::Body>),
    Throttle(Box<throttle::Body>),
    Unthrottle(Box<unthrottle::Body>),
    Fork(Box<fork::Body>),
    Read(Box<read::Body>),
    Sample(Box<sample::Body>),
    #[cfg(feature = "linux-3.12")]
    Mmap2(Box<mmap2::Body>),
    #[cfg(feature = "linux-4.1")]
    Aux(Box<aux::Body>),
    #[cfg(feature = "linux-4.1")]
    ItraceStart(Box<intrace_start::Body>),
    #[cfg(feature = "linux-4.2")]
    LostSamples(Box<lost_samples::Body>),
    #[cfg(feature = "linux-4.3")]
    Switch(Box<switch::Body>),
    #[cfg(feature = "linux-4.3")]
    SwitchCpuWide(Box<switch_cpu_wide::Body>),
    #[cfg(feature = "linux-4.12")]
    Namespaces(Box<namespaces::Body>),
    #[cfg(feature = "linux-5.1")]
    Ksymbol(Box<ksymbol::Body>),
    #[cfg(feature = "linux-5.1")]
    BpfEvent(Box<bpf_event::Body>),
    #[cfg(feature = "linux-5.7")]
    Cgroup(Box<cgroup::Body>),
    #[cfg(feature = "linux-5.9")]
    TextPoke(Box<text_poke::Body>),
    AuxOutputHwId(Box<aux_output_hw_id::Body>), // TODO: missing docs in manual
}
