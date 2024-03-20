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
use crate::counting::{Config, Counter};
use crate::{Event, EventScope};

pub fn test_single<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    test_stat(ev, workload);
    test_enable_disable(ev, workload);
    test_reset(ev, workload);
}

fn gen_counter(ev: &Event) -> Counter {
    let scopes = EventScope::all();
    let mut cfg = Config::new(ev, &scopes);

    Counter::new(&Process::Current, &Cpu::Any, &mut cfg).unwrap()
}

fn test_stat<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut counter = gen_counter(ev);

    let before = counter.stat().unwrap().event_count;
    dbg!(before);
    assert_eq!(before, 0);
    counter.enable().unwrap();

    workload();

    counter.disable().unwrap();
    let after = counter.stat().unwrap().event_count;
    dbg!(after);
    assert!(after > 0);
}

fn test_enable_disable<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut counter = gen_counter(ev);

    counter.enable().unwrap();
    workload();
    counter.disable().unwrap();
    let after = counter.stat().unwrap().event_count;
    assert!(after > 0);

    assert_eq!(after, counter.stat().unwrap().event_count);
    counter.enable().unwrap();
    workload();
    assert!(after < counter.stat().unwrap().event_count);
}

fn test_reset<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut counter = gen_counter(ev);

    counter.enable().unwrap();
    workload();
    counter.disable().unwrap();
    let count = counter.stat().unwrap().event_count;
    assert!(count > 0);

    counter.disable().unwrap();
    counter.reset().unwrap();
    workload();
    assert_eq!(counter.stat().unwrap().event_count, 0);
}
