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

mod breakpoint;
mod tracepoint;

use crate::config::{Cpu, Process};
use crate::tracing::{Config, ExtraConfig, Tracer};
use crate::{Event, EventScope};

fn gen_tracer(cfg: &Config) -> Tracer {
    let mmap_pages = 1 + 512;
    Tracer::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

pub fn gen_cfg(ev: &Event) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.addr = true;
    let scopes = EventScope::all();
    Config::extra_new(ev, &scopes, &extra_config)
}
