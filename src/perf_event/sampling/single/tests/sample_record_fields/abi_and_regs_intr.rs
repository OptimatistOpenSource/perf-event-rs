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
use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy, Sampler};
use crate::test::cpu_workload;
use crate::{Event, EventScope, HardwareEvent};

fn gen_sampler(cfg: &Config) -> Sampler {
    let mmap_pages = 1 + 512;
    Sampler::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

fn gen_cfg(sample_regs_intr: u64) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.abi_and_regs_intr = Some(sample_regs_intr);

    let event = HardwareEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::extra_new(&Event::from(event), &scopes, &overflow_by, &extra_config)
}

#[test]
fn test() {
    for i in 1..7 {
        let cfg = gen_cfg(i);
        let mut sampler = gen_sampler(&cfg);

        sampler.enable().unwrap();
        cpu_workload();
        sampler.disable().unwrap();

        let mut sample_count = 0_usize;
        for Record { body, .. } in sampler.iter() {
            if let RecordBody::Sample(body) = body {
                assert!(body.abi_and_regs_intr.is_some());
                sample_count += 1;
            }
        }
        assert!(sample_count > 0);
    }
}
