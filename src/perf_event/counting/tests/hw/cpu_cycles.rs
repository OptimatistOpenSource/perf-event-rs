use crate::counting::Attr;
use crate::{Builder, EventScope, HwEvent};

fn workload() {
    for _ in 0..10000000 {
        std::hint::black_box(0);
    }
}

// TODO: need refactor
#[test]
fn test() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = HwEvent::CpuCycles;
        let scopes = [EventScope::User, EventScope::Host];
        Attr::new(event, scopes, Default::default())
    };
    let mut counting = builder.build_counting(attr).unwrap();

    let before = counting.result().unwrap().event_count;
    dbg!(before);
    assert_eq!(before, 0);
    counting.enable().unwrap();

    workload();

    counting.disable().unwrap();
    let after = counting.result().unwrap().event_count;
    dbg!(after);
    assert!(after > 0);
    assert_eq!(after, counting.result().unwrap().event_count);

    // restart test
    counting.enable().unwrap();
    assert!(after < counting.result().unwrap().event_count);

    // reset_count test
    counting.disable().unwrap();
    counting.reset_count().unwrap();
    assert_eq!(counting.result().unwrap().event_count, 0);
}
