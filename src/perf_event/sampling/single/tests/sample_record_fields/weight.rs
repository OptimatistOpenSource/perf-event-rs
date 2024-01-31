use crate::config::{Cpu, Process};
use crate::sampling::record::sample::{Weight, WeightRepr};
use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy, Sampler};
use crate::test::cpu_workload;
use crate::{Event, EventScope, HardwareEvent};

fn gen_sampler(cfg: &Config) -> Sampler {
    let mmap_pages = 1 + 512;
    Sampler::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

fn gen_cfg(repr: WeightRepr) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.weight = Some(repr);

    let event = HardwareEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::extra_new(&Event::from(event), &scopes, &overflow_by, &extra_config)
}

#[test]
fn test_full() {
    let cfg = gen_cfg(WeightRepr::Full);
    let mut sampler = gen_sampler(&cfg);

    sampler.enable().unwrap();
    cpu_workload();
    sampler.disable().unwrap();

    let mut sample_count = 0_usize;
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(body) = body {
            assert!(matches!(body.weight, Some(Weight::Full(_))));
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
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(body) = body {
            assert!(matches!(body.weight, Some(Weight::Vars { .. })));
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}
