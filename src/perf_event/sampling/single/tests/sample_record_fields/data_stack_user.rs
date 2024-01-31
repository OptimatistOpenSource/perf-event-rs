use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy, Sampler};
use crate::test::cpu_workload;
use crate::{Event, EventScope, HardwareEvent};
use crate::config::{Cpu, Process};

fn gen_sampler(cfg: &Config) -> Sampler {
    let mmap_pages = 1 + 512;
    Sampler::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

fn gen_cfg(sample_stack_user: u16) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.data_stack_user = Some(sample_stack_user);

    let event = HardwareEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::extra_new(&Event::from(event), &scopes, &overflow_by, &extra_config)
}

#[test]
fn test() {
    for i in 3..8 {
        let cfg = gen_cfg(2_u16.pow(i));
        let mut sampler = gen_sampler(&cfg);

        sampler.enable().unwrap();
        cpu_workload();
        sampler.disable().unwrap();

        let mut sample_count = 0_usize;
        for Record { body, .. } in sampler.iter() {
            if let RecordBody::Sample(body) = body {
                assert!(body.data_stack_user.is_some());
                sample_count += 1;
            }
        }
        assert!(sample_count > 0);
    }
}
