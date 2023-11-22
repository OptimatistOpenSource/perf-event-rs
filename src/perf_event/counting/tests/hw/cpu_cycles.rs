use crate::counting::{Attr, HwEvent};
use crate::{Builder, EventScope};
use std::io;

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
        Attr::new(event, scopes, Default::default())
    };
    let mut counting = builder.build_counting(attr).unwrap();

    let before = counting.get_result().unwrap().event_count;
    dbg!(before);
    assert_eq!(before, 0);
    counting.enable().unwrap();

    workload();

    counting.disable().unwrap();
    let after = counting.get_result().unwrap().event_count;
    dbg!(after);
    assert!(after > 0);
    assert_eq!(after, counting.get_result().unwrap().event_count);

    // restart test
    counting.enable().unwrap();
    assert!(after < counting.get_result().unwrap().event_count);

    // reset_count test
    counting.disable().unwrap();
    counting.reset_count().unwrap();
    assert_eq!(counting.get_result().unwrap().event_count, 0);
}
