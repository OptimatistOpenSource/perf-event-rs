use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, OverflowBy};
use crate::{Builder, EventScope, HwEvent};

fn workload() {
    for _ in 0..10000000 {
        std::hint::black_box(0);
    }
}

#[test]
fn test() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = HwEvent::CpuCycles;
        let scopes = [EventScope::User, EventScope::Host];
        let overflow_by = OverflowBy::Period(1000);
        Attr::new(event, scopes, overflow_by)
    };
    let mut sampling = builder.build_sampling(&attr).unwrap();

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
