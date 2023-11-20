use crate::counting::{Attr, HwEvent, SwEvent};
use crate::{Builder, EventScope};

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
        Attr::new(event, scopes)
    };
    let mut counting = builder.build_counting(attr).unwrap();

    let before = counting.get_count().unwrap();
    dbg!(before);
    assert_eq!(before, 0);
    counting.enable().unwrap();

    workload();

    counting.disable().unwrap();
    let after = counting.get_count().unwrap();
    dbg!(after);
    assert!(after > 0);
    assert_eq!(after, counting.get_count().unwrap());

    counting.enable().unwrap();
    assert_ne!(after, counting.get_count().unwrap());
}
