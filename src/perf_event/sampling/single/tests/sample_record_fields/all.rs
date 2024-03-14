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

use crate::config::{Cpu, Process};
use crate::sampling::record::sample::WeightRepr;
use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy, SampleRecordFields, Sampler};
use crate::test::cpu_workload;
use crate::{Event, EventScope, HardwareEvent};

fn gen_sampler(cfg: &Config) -> Sampler {
    let mmap_pages = 1 + 512;
    Sampler::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

fn gen_cfg(extra_config: ExtraConfig) -> Config {
    let event = HardwareEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::extra_new(&Event::from(event), &scopes, &overflow_by, &extra_config)
}

#[test]
fn test() {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields = SampleRecordFields {
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
        v: true,
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
    let cfg = gen_cfg(extra_config);
    let mut sampler = gen_sampler(&cfg);

    sampler.enable().unwrap();
    cpu_workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(body) = body {
            #[cfg(feature = "linux-3.12")]
            assert!(body.sample_id.is_some());
            assert!(body.ip.is_some());
            assert!(body.pid.is_some());
            assert!(body.tid.is_some());
            assert!(body.time.is_some());
            assert!(body.addr.is_some());
            assert!(body.id.is_some());
            assert!(body.stream_id.is_some());
            assert!(body.cpu.is_some());
            assert!(body.period.is_some());
            assert!(body.v.is_some());
            assert!(body.ips.is_some());
            assert!(body.data_raw.is_some());
            assert!(body.abi_and_regs_user.is_some());
            assert!(body.data_stack_user.is_some());
            assert!(body.weight.is_some());
            assert!(body.data_src.is_some());
            #[cfg(feature = "linux-3.13")]
            assert!(body.transaction.is_some());
            #[cfg(feature = "linux-3.19")]
            assert!(body.abi_and_regs_intr.is_some());
            #[cfg(feature = "linux-4.14")]
            assert!(body.phys_addr.is_some());
            #[cfg(feature = "linux-5.7")]
            assert!(body.cgroup.is_some());
            #[cfg(feature = "linux-5.11")]
            assert!(body.data_page_size.is_some());
            #[cfg(feature = "linux-5.11")]
            assert!(body.code_page_size.is_some());
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}
