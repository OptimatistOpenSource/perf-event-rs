use crate::counting::Attr;
use crate::test::cpu_workload;
use crate::{Builder, EventScope, SwEvent};

#[test]
fn test_basic() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        Attr::new(event, scopes, Default::default())
    };
    let mut counting = builder.build_counting(&attr).unwrap();

    let before = counting.result().unwrap().event_count;
    dbg!(before);
    assert_eq!(before, 0);
    counting.enable().unwrap();

    cpu_workload();

    counting.disable().unwrap();
    let after = counting.result().unwrap().event_count;
    dbg!(after);
    assert!(after > 0);
}

#[test]
fn test_enable_disable() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        Attr::new(event, scopes, Default::default())
    };
    let mut counting = builder.build_counting(&attr).unwrap();

    counting.enable().unwrap();
    cpu_workload();
    counting.disable().unwrap();
    let after = counting.result().unwrap().event_count;
    assert!(after > 0);

    assert_eq!(after, counting.result().unwrap().event_count);
    counting.enable().unwrap();
    cpu_workload();
    assert!(after < counting.result().unwrap().event_count);
}

#[test]
fn test_reset_count() {
    let builder = Builder::new().calling_process().any_cpu();
    let attr = {
        let event = SwEvent::CpuClock;
        let scopes = [EventScope::User, EventScope::Host];
        Attr::new(event, scopes, Default::default())
    };
    let mut counting = builder.build_counting(&attr).unwrap();

    counting.enable().unwrap();
    cpu_workload();
    counting.disable().unwrap();
    let count = counting.result().unwrap().event_count;
    assert!(count > 0);

    counting.disable().unwrap();
    counting.reset_count().unwrap();
    cpu_workload();
    assert_eq!(counting.result().unwrap().event_count, 0);
}
