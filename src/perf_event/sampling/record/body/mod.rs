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
pub mod intrace_start;
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
