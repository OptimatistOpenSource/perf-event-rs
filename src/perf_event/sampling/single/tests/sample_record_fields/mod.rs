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

#[cfg(feature = "linux-3.19")]
mod abi_and_regs_intr;
mod abi_and_regs_user;
mod all;
mod data_stack_user;
mod ips;
mod weight;

use crate::config::{Cpu, Process};
use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy, Sampler};
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

macro_rules! gen_test {
    ($field: ident) => {
        #[test]
        fn $field() {
            let mut extra_config = ExtraConfig::default();
            extra_config.sample_record_fields.$field = true;

            let cfg = gen_cfg(extra_config);
            let mut sampler = gen_sampler(&cfg);

            sampler.enable().unwrap();
            cpu_workload();
            sampler.disable().unwrap();

            let mut sample_count = 0_usize;
            for Record { body, .. } in sampler.iter() {
                if let RecordBody::Sample(body) = body {
                    assert!(body.$field.is_some());
                    sample_count += 1;
                }
            }
            assert!(sample_count > 0);
        }
    };
}

#[cfg(feature = "linux-3.12")]
gen_test!(sample_id);
gen_test!(ip);

#[test]
fn pid_and_tid() {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.pid_and_tid = true;

    let cfg = gen_cfg(extra_config);
    let mut sampler = gen_sampler(&cfg);

    sampler.enable().unwrap();
    cpu_workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(body) = body {
            assert!(body.pid.is_some());
            assert!(body.tid.is_some());
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}

gen_test!(time);
gen_test!(addr);
gen_test!(id);
gen_test!(stream_id);
gen_test!(cpu);
gen_test!(period);
gen_test!(v);
gen_test!(data_raw);
gen_test!(data_src);
#[cfg(feature = "linux-3.13")]
gen_test!(transaction);
#[cfg(feature = "linux-4.14")]
gen_test!(phys_addr);
#[cfg(feature = "linux-5.7")]
gen_test!(cgroup);
#[cfg(feature = "linux-5.11")]
gen_test!(data_page_size);
#[cfg(feature = "linux-5.11")]
gen_test!(code_page_size);
