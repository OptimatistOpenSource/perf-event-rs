use crate::counting::{Config, Counter};
use crate::test::cpu_workload;
use crate::{Builder, Event, EventScope, SoftwareEvent};

fn gen_counting() -> Counter {
    let builder = Builder::new().calling_process().any_cpu();

    let event = SoftwareEvent::CpuClock;
    let scopes = [EventScope::User, EventScope::Host];
    let cfg = Config::new(&Event::from(event), &scopes, &Default::default());

    builder.build_counting(&cfg).unwrap()
}

#[test]
fn test_basic() {
    let mut counter = gen_counting();

    let before = counter.result().unwrap().event_count;
    dbg!(before);
    assert_eq!(before, 0);
    counter.enable().unwrap();

    cpu_workload();

    counter.disable().unwrap();
    let after = counter.result().unwrap().event_count;
    dbg!(after);
    assert!(after > 0);
}

#[test]
fn test_enable_disable() {
    let mut counter = gen_counting();

    counter.enable().unwrap();
    cpu_workload();
    counter.disable().unwrap();
    let after = counter.result().unwrap().event_count;
    assert!(after > 0);

    assert_eq!(after, counter.result().unwrap().event_count);
    counter.enable().unwrap();
    cpu_workload();
    assert!(after < counter.result().unwrap().event_count);
}

#[test]
fn test_reset_count() {
    let mut counter = gen_counting();

    counter.enable().unwrap();
    cpu_workload();
    counter.disable().unwrap();
    let count = counter.result().unwrap().event_count;
    assert!(count > 0);

    counter.disable().unwrap();
    counter.reset_count().unwrap();
    cpu_workload();
    assert_eq!(counter.result().unwrap().event_count, 0);
}
