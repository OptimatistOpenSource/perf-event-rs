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

mod hardware;
mod sample_record_fields;
mod software;

use crate::config::{Cpu, Process};
use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy, Sampler};
use crate::{Event, EventScope};

pub fn test_single<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    test_next_record(ev, workload);
    test_all_records(ev, workload);
    test_enable_disable(ev, workload);
    #[cfg(feature = "linux-4.7")]
    test_pause_resume(ev, workload);
    test_ring_buffer(ev, workload);
    test_stat(ev, workload);
}

fn gen_sampler(cfg: &Config) -> Sampler {
    let mmap_pages = 1 + 512;
    Sampler::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

fn gen_cfg(ev: &Event) -> Config {
    let scopes = EventScope::all();
    let overflow_by = OverflowBy::Period(1000);
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.time = true;
    Config::extra_new(&ev, &scopes, &overflow_by, &extra_config)
}

fn test_next_record<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut sampler = gen_sampler(&gen_cfg(ev));

    sampler.enable().unwrap();
    workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    let mut last_time = 0;
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(sample) = body {
            assert!(sample.time.unwrap() >= last_time);
            last_time = sample.time.unwrap();
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}

fn test_all_records<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut sampler = gen_sampler(&gen_cfg(ev));

    sampler.enable().unwrap();
    workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    let mut last_time = 0;
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(sample) = body {
            assert!(sample.time.unwrap() >= last_time);
            last_time = sample.time.unwrap();
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}

fn test_enable_disable<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut sampler = gen_sampler(&gen_cfg(ev));

    assert!(sampler.next_record().is_none());
    sampler.enable().unwrap();
    workload();
    sampler.disable().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in sampler.iter() {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    workload();
    assert!(sampler.next_record().is_none());

    sampler.enable().unwrap();
    workload();
    assert!(sampler.next_record().is_some());
}

#[cfg(feature = "linux-4.7")]
fn test_pause_resume<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut sampler = gen_sampler(&gen_cfg(ev));

    assert!(sampler.next_record().is_none());
    sampler.enable().unwrap();
    workload();
    sampler.pause().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in sampler.iter() {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    workload();
    assert!(sampler.next_record().is_none());

    sampler.resume().unwrap();
    workload();
    assert!(sampler.next_record().is_some());
}

fn test_ring_buffer<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut sampler = gen_sampler(&gen_cfg(ev));

    sampler.enable().unwrap();
    workload();

    let mut sample_count = 0_usize;
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(_) = body {
            sample_count += 1;
        }
    }

    assert!(sample_count > 10100);
}

fn test_stat<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut sampler = gen_sampler(&gen_cfg(ev));

    sampler.enable().unwrap();
    workload();
    sampler.disable().unwrap();

    let stat = sampler.stat().unwrap();
    assert!(stat.event_count > 0);
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
}
