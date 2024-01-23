use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, Event, EventScope, HardwareEvent};

fn gen_builder(mmap_pages: usize) -> Builder {
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_cfg() -> Config {
    let event = HardwareEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.time = true;
    Config::new(&Event::from(event), &scopes, &overflow_by, &extra_config)
}

#[test]
fn test_basic() {
    let builder = gen_builder(1 + (1 << 16));
    let cfg = gen_cfg();
    let mut sampler = builder.build_sampling(&cfg).unwrap();

    sampler.enable().unwrap();
    cpu_workload();
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

#[test]
fn test_all_records() {
    let builder = gen_builder(1 + (1 << 16));
    let cfg = gen_cfg();
    let mut sampler = builder.build_sampling(&cfg).unwrap();

    sampler.enable().unwrap();
    cpu_workload();
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

#[test]
fn test_enable_disable() {
    let builder = gen_builder(1 + (1 << 16));
    let cfg = gen_cfg();
    let mut sampler = builder.build_sampling(&cfg).unwrap();

    assert!(sampler.next_record().is_none());
    sampler.enable().unwrap();
    cpu_workload();
    sampler.disable().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in sampler.iter() {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    cpu_workload();
    assert!(sampler.next_record().is_none());

    sampler.enable().unwrap();
    cpu_workload();
    assert!(sampler.next_record().is_some());
}

#[test]
fn test_pause_resume() {
    let builder = gen_builder(1 + (1 << 16));
    let cfg = gen_cfg();
    let mut sampler = builder.build_sampling(&cfg).unwrap();

    assert!(sampler.next_record().is_none());
    sampler.enable().unwrap();
    cpu_workload();
    sampler.pause().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in sampler.iter() {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    cpu_workload();
    assert!(sampler.next_record().is_none());

    sampler.resume().unwrap();
    cpu_workload();
    assert!(sampler.next_record().is_some());
}

#[test]
fn test_ring_buffer() {
    let builder = gen_builder(1 + 512);
    let cfg = gen_cfg();
    let mut sampler = builder.build_sampling(&cfg).unwrap();

    sampler.enable().unwrap();
    cpu_workload();

    let mut sample_count = 0_usize;
    for Record { body, .. } in sampler.iter() {
        if let RecordBody::Sample(_) = body {
            sample_count += 1;
        }
    }

    assert!(sample_count > 10100);
}
