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

use crate::sampling::record::{Record, RecordBody};
use crate::test::{cpu_workload, read_file};
use crate::tracing::tests::{gen_cfg, gen_tracer};
use crate::{Event, TracepointEvent};
use std::str::FromStr;

fn test<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    test_next_record(ev, workload);
    test_stat(ev, workload);
}

fn test_next_record<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut tracer = gen_tracer(&gen_cfg(ev));

    tracer.enable().unwrap();
    workload();
    tracer.disable().unwrap();

    let mut sample_count = 0;
    for Record { body, .. } in tracer.iter() {
        if let RecordBody::Sample(body) = body {
            sample_count += 1;
            assert!(body.addr.is_some());
        }
    }
    assert!(sample_count > 0);
}

fn test_stat<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut tracer = gen_tracer(&gen_cfg(ev));

    tracer.enable().unwrap();
    workload();
    tracer.disable().unwrap();

    let stat = tracer.stat().unwrap();
    assert!(stat.event_count > 0);
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
}

#[test]
fn test_sched_switch() {
    let id = {
        let path = "/sys/kernel/debug/tracing/events/sched/sched_switch/id";
        let string = read_file(path).replace('\n', "");
        u64::from_str(string.as_str()).unwrap()
    };
    let ev = TracepointEvent::new(id);

    let mut workload = cpu_workload;

    test(&Event::from(ev), &mut workload);
}
