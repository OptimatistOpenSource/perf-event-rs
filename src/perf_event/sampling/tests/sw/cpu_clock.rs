use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, ExtraRecord, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, SwEvent};

#[test]
fn test_basic() {
    let mmap_pages = 1 + (1 << 16);
    let builder = Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages);
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by, &Default::default(), [])
    };
    let sampling = builder.build_sampling(&attr).unwrap();

    sampling.enable().unwrap();
    cpu_workload();
    sampling.disable().unwrap();

    let mut sample_count = 0_usize;
    let mut last_time = 0;
    for Record { body, .. } in sampling {
        if let RecordBody::Sample(sample) = body {
            assert!(sample.time >= last_time);
            last_time = sample.time;
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}

#[test]
fn test_all_records() {
    let mmap_pages = 1 + (1 << 16);
    let builder = Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages);
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(
            event,
            scopes,
            overflow_by,
            &Default::default(),
            ExtraRecord::all(),
        )
    };
    let sampling = builder.build_sampling(&attr).unwrap();

    sampling.enable().unwrap();
    cpu_workload();
    sampling.disable().unwrap();

    let mut sample_count = 0_usize;
    let mut last_time = 0;
    for Record { body, .. } in sampling {
        if let RecordBody::Sample(sample) = body {
            assert!(sample.time >= last_time);
            last_time = sample.time;
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}

#[test]
fn test_enable_disable() {
    let mmap_pages = 1 + (1 << 16);
    let builder = Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages);
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by, &Default::default(), [])
    };
    let mut sampling = builder.build_sampling(&attr).unwrap();

    assert!(sampling.next_record().is_none());
    sampling.enable().unwrap();
    cpu_workload();
    sampling.disable().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in &mut sampling {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    cpu_workload();
    assert!(sampling.next_record().is_none());

    sampling.enable().unwrap();
    cpu_workload();
    assert!(sampling.next_record().is_some());
}

#[test]
fn test_pause_resume() {
    let mmap_pages = 1 + (1 << 16);
    let builder = Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages);
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by, &Default::default(), [])
    };
    let mut sampling = builder.build_sampling(&attr).unwrap();

    assert!(sampling.next_record().is_none());
    sampling.enable().unwrap();
    cpu_workload();
    sampling.pause().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in &mut sampling {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    cpu_workload();
    assert!(sampling.next_record().is_none());

    sampling.resume().unwrap();
    cpu_workload();
    assert!(sampling.next_record().is_some());
}

#[test]
fn test_ring_buffer() {
    let mmap_pages = 1 + 512;
    let builder = Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages);
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1);
        Attr::new(event, scopes, overflow_by, &Default::default(), [])
    };
    let mut sampling = builder.build_sampling(&attr).unwrap();

    sampling.enable().unwrap();
    cpu_workload();

    let mut sample_count = 0_usize;
    for Record { body, .. } in &mut sampling {
        if let RecordBody::Sample(_) = body {
            sample_count += 1;
        }
    }

    assert!(sample_count > 10100);
}
