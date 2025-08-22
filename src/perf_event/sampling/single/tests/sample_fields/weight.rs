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
use crate::sampling::record::sample::{Weight, WeightRepr};
use crate::sampling::record::Record;
use crate::sampling::{EventConfig, OverflowBy, Sampler, SamplerConfig};
use crate::test::cpu_workload;
use crate::{Event, EventScope, HardwareEvent};

fn gen_sampler(cfg: &EventConfig) -> Sampler {
    let mmap_pages = 1 + 512;
    Sampler::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

fn gen_cfg(repr: WeightRepr) -> EventConfig {
    let mut sampler_config = SamplerConfig::default();
    sampler_config.sample_fields.weight = Some(repr);

    let event = HardwareEvent::CpuCycles;
    let scopes = EventScope::all();
    let overflow_by = OverflowBy::Period(1000);
    EventConfig::new_with_sampler_config(
        &Event::from(event),
        &scopes,
        &overflow_by,
        &sampler_config,
    )
}

#[test]
fn test_full() {
    let cfg = gen_cfg(WeightRepr::Full);
    let mut sampler = gen_sampler(&cfg);

    sampler.enable().unwrap();
    cpu_workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    for record in sampler.iter() {
        if let Record::Sample(sample_record) = record {
            assert!(matches!(sample_record.weight, Some(Weight::Full(_))));
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}

#[cfg(feature = "linux-5.12")]
#[test]
fn test_vars() {
    let cfg = gen_cfg(WeightRepr::Vars);
    let mut sampler = gen_sampler(&cfg);

    sampler.enable().unwrap();
    cpu_workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    for record in sampler.iter() {
        if let Record::Sample(sample_record) = record {
            assert!(matches!(sample_record.weight, Some(Weight::Vars { .. })));
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}
