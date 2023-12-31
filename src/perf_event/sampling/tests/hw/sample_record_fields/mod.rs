mod abi_and_regs_intr;
mod abi_and_regs_user;
mod all;
mod data_stack_user;
mod ips;
mod weight;

use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, HwEvent};

fn gen_builder() -> Builder {
    let mmap_pages = 1 + 512;
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_cfg(extra_config: ExtraConfig) -> Config {
    let event = HwEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::new(event, scopes, overflow_by, &extra_config)
}

macro_rules! gen_test {
    ($field: ident) => {
        #[test]
        fn $field() {
            let mut extra_config = ExtraConfig::default();
            extra_config.sample_record_fields.$field = true;

            let builder = gen_builder();
            let cfg = gen_cfg(extra_config);

            let mut sampler = builder.build_sampling(&cfg).unwrap();
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

gen_test!(sample_id);
gen_test!(ip);

#[test]
fn pid_and_tid() {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.pid_and_tid = true;

    let builder = gen_builder();
    let cfg = gen_cfg(extra_config);

    let mut sampler = builder.build_sampling(&cfg).unwrap();
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
gen_test!(transaction);
gen_test!(phys_addr);
gen_test!(cgroup);
gen_test!(data_page_size);
gen_test!(code_page_size);
