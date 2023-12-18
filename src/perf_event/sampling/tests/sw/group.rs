use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, FixedSamplingGroup, OverflowBy, SamplingGuard};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, SwEvent};

fn gen_builder() -> Builder {
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(1 + 512)
}

fn gen_cfg(event: SwEvent) -> Config {
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    let extra_config = ExtraConfig::default();
    Config::new(event, scopes, overflow_by, &extra_config)
}

#[test]
fn test_basic() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let cpu_clock_guard1 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let cpu_clock_guard2 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();

    assert!(group.next_record(&cpu_clock_guard1).is_none());
    assert!(group.next_record(&cpu_clock_guard2).is_none());

    let mut group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let mut cpu_clock_sample_count = 0;
    let mut next = group.next_record(&cpu_clock_guard1);
    while let Some(record) = next {
        if let RecordBody::Sample(_) = record.body {
            cpu_clock_sample_count += 1;
        }
        next = group.next_record(&cpu_clock_guard1);
    }
    assert!(cpu_clock_sample_count > 0);

    let mut cpu_clock_sample_count = 0;
    let mut next = group.next_record(&cpu_clock_guard2);
    while let Some(record) = next {
        if let RecordBody::Sample(_) = record.body {
            cpu_clock_sample_count += 1;
        }
        next = group.next_record(&cpu_clock_guard2);
    }
    assert!(cpu_clock_sample_count > 0);
}

#[test]
fn test_enable_disable() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let cpu_clock_guard1 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let cpu_clock_guard2 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();

    assert!(group.next_record(&cpu_clock_guard1).is_none());
    assert!(group.next_record(&cpu_clock_guard2).is_none());

    let mut group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    fn consume_records(group: &mut FixedSamplingGroup, guard: &SamplingGuard) {
        let mut count = 0;
        let mut next = group.next_record(&guard);
        while let Some(_) = next {
            next = group.next_record(&guard);
            count += 1;
        }
        assert!(count > 0);
    }

    consume_records(&mut group, &cpu_clock_guard1);
    consume_records(&mut group, &cpu_clock_guard2);

    group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    consume_records(&mut group, &cpu_clock_guard1);
    consume_records(&mut group, &cpu_clock_guard2);
}

#[test]
fn test_guard_basic() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let mut cpu_clock_guard1 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let mut cpu_clock_guard2 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();

    assert!(cpu_clock_guard1.next_record().is_none());
    assert!(cpu_clock_guard2.next_record().is_none());

    let group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let mut cpu_clock_sample_count = 0;
    for Record { body, .. } in &mut cpu_clock_guard1 {
        if let RecordBody::Sample(_) = body {
            cpu_clock_sample_count += 1;
        }
    }
    assert!(cpu_clock_sample_count > 0);

    let mut cpu_clock_sample_count = 0;
    for Record { body, .. } in &mut cpu_clock_guard2 {
        if let RecordBody::Sample(_) = body {
            cpu_clock_sample_count += 1;
        }
    }
    assert!(cpu_clock_sample_count > 0);
}

#[test]
fn test_guard_enable_disable() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let mut cpu_clock_guard1 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let mut cpu_clock_guard2 = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();

    assert!(cpu_clock_guard1.next_record().is_none());
    assert!(cpu_clock_guard2.next_record().is_none());

    let group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    fn consume_records(guard: &mut SamplingGuard) {
        let mut count = 0;
        for Record { body, .. } in guard {
            if let RecordBody::Sample(_) = body {
                count += 1;
            }
        }
        assert!(count > 0);
    }

    consume_records(&mut cpu_clock_guard1);
    consume_records(&mut cpu_clock_guard2);

    group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    consume_records(&mut cpu_clock_guard1);
    consume_records(&mut cpu_clock_guard2);
}
