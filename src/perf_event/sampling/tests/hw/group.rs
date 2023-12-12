use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, ExtraConfig, FixedSamplingGroup, OverflowBy, SamplingGuard};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, HwEvent};

fn gen_builder() -> Builder {
    Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(1 + 512)
}

fn gen_attr(event: HwEvent) -> Attr {
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    let extra_config = ExtraConfig::default();
    Attr::new(event, scopes, overflow_by, &extra_config, [])
}

#[test]
fn test_basic() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let cpu_cycles_guard = group.add_member(&gen_attr(HwEvent::CpuCycles)).unwrap();
    let instructions_guard = group.add_member(&gen_attr(HwEvent::Instructions)).unwrap();

    assert!(group.next_record(&cpu_cycles_guard).is_none());
    assert!(group.next_record(&instructions_guard).is_none());

    let mut group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let mut cpu_cycles_sample_count = 0;
    let mut next = group.next_record(&cpu_cycles_guard);
    while let Some(record) = next {
        if let RecordBody::Sample(_) = record.body {
            cpu_cycles_sample_count += 1;
        }
        next = group.next_record(&cpu_cycles_guard);
    }
    assert!(cpu_cycles_sample_count > 0);

    let mut instructions_sample_count = 0;
    let mut next = group.next_record(&instructions_guard);
    while let Some(record) = next {
        if let RecordBody::Sample(_) = record.body {
            instructions_sample_count += 1;
        }
        next = group.next_record(&instructions_guard);
    }
    assert!(instructions_sample_count > 0);
}

#[test]
fn test_enable_disable() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let cpu_cycles_guard = group.add_member(&gen_attr(HwEvent::CpuCycles)).unwrap();
    let instructions_guard = group.add_member(&gen_attr(HwEvent::Instructions)).unwrap();

    assert!(group.next_record(&cpu_cycles_guard).is_none());
    assert!(group.next_record(&instructions_guard).is_none());

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

    consume_records(&mut group, &cpu_cycles_guard);
    consume_records(&mut group, &instructions_guard);

    group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    consume_records(&mut group, &cpu_cycles_guard);
    consume_records(&mut group, &instructions_guard);
}

#[test]
fn test_guard_basic() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let mut cpu_cycles_guard = group.add_member(&gen_attr(HwEvent::CpuCycles)).unwrap();
    let mut instructions_guard = group.add_member(&gen_attr(HwEvent::Instructions)).unwrap();

    assert!(cpu_cycles_guard.next_record().is_none());
    assert!(instructions_guard.next_record().is_none());

    let group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let mut cpu_cycles_sample_count = 0;
    for Record { body, .. } in &mut cpu_cycles_guard {
        if let RecordBody::Sample(_) = body {
            cpu_cycles_sample_count += 1;
        }
    }
    assert!(cpu_cycles_sample_count > 0);

    let mut instructions_sample_count = 0;
    for Record { body, .. } in &mut instructions_guard {
        if let RecordBody::Sample(_) = body {
            instructions_sample_count += 1;
        }
    }
    assert!(instructions_sample_count > 0);
}

#[test]
fn test_guard_enable_disable() {
    let builder = gen_builder();
    let mut group = builder.build_sampling_group().unwrap();
    let mut cpu_cycles_guard = group.add_member(&gen_attr(HwEvent::CpuCycles)).unwrap();
    let mut instructions_guard = group.add_member(&gen_attr(HwEvent::Instructions)).unwrap();

    assert!(cpu_cycles_guard.next_record().is_none());
    assert!(instructions_guard.next_record().is_none());

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

    consume_records(&mut cpu_cycles_guard);
    consume_records(&mut instructions_guard);

    group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    consume_records(&mut cpu_cycles_guard);
    consume_records(&mut instructions_guard);
}
