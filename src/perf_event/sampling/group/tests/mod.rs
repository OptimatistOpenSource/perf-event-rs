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
mod software;

use crate::config::{Cpu, Process};
use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, FixedSamplerGroup, OverflowBy, SamplerGroup, SamplerGuard};
use crate::{Event, EventScope};

pub fn test_group<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    test_next_record(ev_1, ev_2, workload);
    test_enable_disable(ev_1, ev_2, workload);
    test_guard_basic(ev_1, ev_2, workload);
    test_guard_enable_disable(ev_1, ev_2, workload);
    test_stat(ev_1, ev_2, workload);
    test_guard_stat(ev_1, ev_2, workload);
}

fn gen_group() -> SamplerGroup {
    let mmap_pages = 1 + 512;
    SamplerGroup::new(&Process::Current, &Cpu::Any, mmap_pages).unwrap()
}

fn gen_cfg(ev: &Event) -> Config {
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::new(&ev, &scopes, &overflow_by)
}

fn test_next_record<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    assert!(group.next_record(&ev_1_guard).is_none());
    assert!(group.next_record(&ev_2_guard).is_none());

    let mut group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    let mut ev_1_sample_count = 0;
    let mut next = group.next_record(&ev_1_guard);
    while let Some(record) = next {
        if let RecordBody::Sample(_) = record.body {
            ev_1_sample_count += 1;
        }
        next = group.next_record(&ev_1_guard);
    }
    assert!(ev_1_sample_count > 0);

    let mut ev_2_sample_count = 0;
    let mut next = group.next_record(&ev_2_guard);
    while let Some(record) = next {
        if let RecordBody::Sample(_) = record.body {
            ev_2_sample_count += 1;
        }
        next = group.next_record(&ev_2_guard);
    }
    assert!(ev_2_sample_count > 0);
}

fn test_enable_disable<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    assert!(group.next_record(&ev_1_guard).is_none());
    assert!(group.next_record(&ev_2_guard).is_none());

    let mut group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    fn consume_records(group: &mut FixedSamplerGroup, guard: &SamplerGuard) {
        let mut count = 0;
        let mut next = group.next_record(&guard);
        while let Some(_) = next {
            next = group.next_record(&guard);
            count += 1;
        }
        assert!(count > 0);
    }

    consume_records(&mut group, &ev_1_guard);
    consume_records(&mut group, &ev_2_guard);

    group.enable().unwrap();
    workload();
    group.disable().unwrap();

    consume_records(&mut group, &ev_1_guard);
    consume_records(&mut group, &ev_2_guard);
}

fn test_guard_basic<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let mut ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let mut ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    assert!(ev_1_guard.next_record().is_none());
    assert!(ev_2_guard.next_record().is_none());

    let group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    let mut ev_1_sample_count = 0;
    for Record { body, .. } in &mut ev_1_guard {
        if let RecordBody::Sample(_) = body {
            ev_1_sample_count += 1;
        }
    }
    assert!(ev_1_sample_count > 0);

    let mut ev_2_sample_count = 0;
    for Record { body, .. } in &mut ev_2_guard {
        if let RecordBody::Sample(_) = body {
            ev_2_sample_count += 1;
        }
    }
    assert!(ev_2_sample_count > 0);
}

fn test_guard_enable_disable<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let mut ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let mut ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    assert!(ev_1_guard.next_record().is_none());
    assert!(ev_2_guard.next_record().is_none());

    let group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    fn consume_records(guard: &mut SamplerGuard) {
        let mut count = 0;
        for Record { body, .. } in guard {
            if let RecordBody::Sample(_) = body {
                count += 1;
            }
        }
        assert!(count > 0);
    }

    consume_records(&mut ev_1_guard);
    consume_records(&mut ev_2_guard);

    group.enable().unwrap();
    workload();
    group.disable().unwrap();

    consume_records(&mut ev_1_guard);
    consume_records(&mut ev_2_guard);
}

fn test_stat<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    let stat = group.stat().unwrap();
    assert_eq!(stat.time_enabled, 0);
    assert_eq!(stat.time_running, 0);
    assert_eq!(stat.member_count(&ev_1_guard).unwrap().event_count, 0);
    #[cfg(feature = "linux-6.0")]
    assert_eq!(stat.member_count(&ev_1_guard).unwrap().event_lost, 0);
    assert_eq!(stat.member_count(&ev_2_guard).unwrap().event_count, 0);
    #[cfg(feature = "linux-6.0")]
    assert_eq!(stat.member_count(&ev_2_guard).unwrap().event_lost, 0);

    let mut group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    let stat = group.stat().unwrap();
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
    assert!(stat.member_count(&ev_1_guard).unwrap().event_count > 0);
    assert!(stat.member_count(&ev_2_guard).unwrap().event_count > 0);
}

fn test_guard_stat<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let mut ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let mut ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    let ev_1_stat = ev_1_guard.stat().unwrap();
    assert_eq!(ev_1_stat.event_count, 0);
    #[cfg(feature = "linux-6.0")]
    assert_eq!(ev_1_stat.event_lost, 0);
    assert_eq!(ev_1_stat.time_enabled, 0);
    assert_eq!(ev_1_stat.time_running, 0);
    let ev_2_stat = ev_2_guard.stat().unwrap();
    assert_eq!(ev_2_stat.event_count, 0);
    #[cfg(feature = "linux-6.0")]
    assert_eq!(ev_2_stat.event_lost, 0);
    assert_eq!(ev_2_stat.time_enabled, 0);
    assert_eq!(ev_2_stat.time_running, 0);

    let group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    let ev_1_stat = ev_1_guard.stat().unwrap();
    assert!(ev_1_stat.event_count > 0);
    assert!(ev_1_stat.time_enabled > 0);
    assert!(ev_1_stat.time_running > 0);
    let ev_2_stat = ev_2_guard.stat().unwrap();
    assert!(ev_2_stat.event_count > 0);
    assert!(ev_2_stat.time_enabled > 0);
    assert!(ev_2_stat.time_running > 0);
}
