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

use crate::config::{Cpu, Process};
use crate::sampling::record::sample::WeightRepr;
use crate::sampling::record::Record;
use crate::sampling::{EventConfig, OverflowBy, SampleFields, Sampler, SamplerConfig};
use crate::test::cpu_workload;
use crate::{Event, EventScope, HardwareEvent};

fn gen_sampler(cfg: &EventConfig) -> Sampler {
    let mmap_pages = 1 + 512;
    Sampler::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

fn gen_cfg(extra_config: SamplerConfig) -> EventConfig {
    let event = HardwareEvent::CpuCycles;
    let scopes = EventScope::all();
    let overflow_by = OverflowBy::Period(1000);
    EventConfig::new_with_sampler_config(&Event::from(event), &scopes, &overflow_by, &extra_config)
}

#[test]
fn test() {
    let mut sampler_config = SamplerConfig::default();
    sampler_config.sample_fields = SampleFields {
        #[cfg(feature = "linux-3.12")]
        sample_id: true,
        ip: true,
        pid_and_tid: true,
        time: true,
        addr: true,
        id: true,
        stream_id: true,
        cpu: true,
        period: true,
        read: true,
        ips: Some(1),
        data_raw: true,
        abi_and_regs_user: Some(1),
        data_stack_user: Some(2_u16.pow(3)),
        weight: Some(WeightRepr::Full),
        data_src: true,
        #[cfg(feature = "linux-3.13")]
        transaction: true,
        #[cfg(feature = "linux-3.19")]
        abi_and_regs_intr: Some(1),
        #[cfg(feature = "linux-4.14")]
        phys_addr: true,
        #[cfg(feature = "linux-5.7")]
        cgroup: true,
        #[cfg(feature = "linux-5.11")]
        data_page_size: true,
        #[cfg(feature = "linux-5.11")]
        code_page_size: true,
    };
    let ev_cfg = gen_cfg(sampler_config);
    let mut sampler = gen_sampler(&ev_cfg);

    sampler.enable().unwrap();
    cpu_workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    for record in sampler.iter() {
        if let Record::Sample(sample_record) = record {
            #[cfg(feature = "linux-3.12")]
            assert!(sample_record.sample_id.is_some());
            assert!(sample_record.ip.is_some());
            assert!(sample_record.pid.is_some());
            assert!(sample_record.tid.is_some());
            assert!(sample_record.time.is_some());
            assert!(sample_record.addr.is_some());
            assert!(sample_record.id.is_some());
            assert!(sample_record.stream_id.is_some());
            assert!(sample_record.cpu.is_some());
            assert!(sample_record.period.is_some());
            assert!(sample_record.read.is_some());
            assert!(sample_record.ips.is_some());
            assert!(sample_record.data_raw.is_some());
            assert!(sample_record.abi_and_regs_user.is_some());
            assert!(sample_record.data_stack_user.is_some());
            assert!(sample_record.weight.is_some());
            assert!(sample_record.data_src.is_some());
            #[cfg(feature = "linux-3.13")]
            assert!(sample_record.transaction.is_some());
            #[cfg(feature = "linux-3.19")]
            assert!(sample_record.abi_and_regs_intr.is_some());
            #[cfg(feature = "linux-4.14")]
            assert!(sample_record.phys_addr.is_some());
            #[cfg(feature = "linux-5.7")]
            assert!(sample_record.cgroup.is_some());
            #[cfg(feature = "linux-5.11")]
            assert!(sample_record.data_page_size.is_some());
            #[cfg(feature = "linux-5.11")]
            assert!(sample_record.code_page_size.is_some());
            sample_count += 1;
            println!("body: {}", sample_record);
        }
    }
    println!("sample_count: {}", sample_count);
    assert!(sample_count > 0);
}
