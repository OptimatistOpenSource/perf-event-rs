use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, ExtraRecord, OverflowBy};
use crate::{Builder, EventScope, SwEvent};

fn workload() {
    for _ in 0..10000000 {
        std::hint::black_box(0);
    }
}

#[test]
fn test() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by, [])
    };
    let sampling = builder.build_sampling(&attr).unwrap();

    sampling.enable().unwrap();
    workload();
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
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by, ExtraRecord::all())
    };
    let sampling = builder.build_sampling(&attr).unwrap();

    sampling.enable().unwrap();
    workload();
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
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by, [])
    };
    let mut sampling = builder.build_sampling(&attr).unwrap();

    assert!(sampling.next_sample().is_none());
    sampling.enable().unwrap();
    workload();
    sampling.disable().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in &mut sampling {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    workload();
    assert!(sampling.next_sample().is_none());

    sampling.enable().unwrap();
    workload();
    assert!(sampling.next_sample().is_some());
}

#[test]
fn test_pause_resume() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by, [])
    };
    let mut sampling = builder.build_sampling(&attr).unwrap();

    assert!(sampling.next_sample().is_none());
    sampling.enable().unwrap();
    workload();
    sampling.pause().unwrap();

    {
        let mut sample_count = 0_usize;
        for _ in &mut sampling {
            sample_count += 1;
        }
        assert!(sample_count > 0);
    }

    workload();
    assert!(sampling.next_sample().is_none());

    sampling.resume().unwrap();
    workload();
    assert!(sampling.next_sample().is_some());
}
