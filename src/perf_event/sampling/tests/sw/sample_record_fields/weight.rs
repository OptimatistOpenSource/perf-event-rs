use crate::sampling::record::sample::{Weight, WeightRepr};
use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, SoftwareEvent};

fn gen_builder() -> Builder {
    let mmap_pages = 1 + 512;
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_cfg(repr: WeightRepr) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.weight = Some(repr);

    let event = SoftwareEvent::CpuClock;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::new(event, scopes, overflow_by, &extra_config)
}

#[test]
fn test_full() {
    let builder = gen_builder();
    let cfg = gen_cfg(WeightRepr::Full);
    let mut sampler = builder.build_sampling(&cfg).unwrap();

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

#[test]
fn test_vars() {
    let builder = gen_builder();
    let cfg = gen_cfg(WeightRepr::Vars);
    let mut sampler = builder.build_sampling(&cfg).unwrap();

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
