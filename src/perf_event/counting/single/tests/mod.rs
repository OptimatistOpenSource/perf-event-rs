mod hardware;
mod software;

use crate::counting::{Config, Counter};
use crate::{Builder, Event, EventScope};

pub fn test_single<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    test_stat(ev, workload);
    test_enable_disable(ev, workload);
    test_reset_count(ev, workload);
}

fn gen_counting(ev: &Event) -> Counter {
    let builder = Builder::new().calling_process().any_cpu();

    let scopes = [EventScope::User, EventScope::Host];
    let cfg = Config::new(ev, &scopes, &Default::default());

    builder.build_counting(&cfg).unwrap()
}

fn test_stat<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut counter = gen_counting(ev);

    let before = counter.stat().unwrap().event_count;
    dbg!(before);
    assert_eq!(before, 0);
    counter.enable().unwrap();

    workload();

    counter.disable().unwrap();
    let after = counter.stat().unwrap().event_count;
    dbg!(after);
    assert!(after > 0);
}

fn test_enable_disable<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut counter = gen_counting(ev);

    counter.enable().unwrap();
    workload();
    counter.disable().unwrap();
    let after = counter.stat().unwrap().event_count;
    assert!(after > 0);

    assert_eq!(after, counter.stat().unwrap().event_count);
    counter.enable().unwrap();
    workload();
    assert!(after < counter.stat().unwrap().event_count);
}

fn test_reset_count<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut counter = gen_counting(ev);

    counter.enable().unwrap();
    workload();
    counter.disable().unwrap();
    let count = counter.stat().unwrap().event_count;
    assert!(count > 0);

    counter.disable().unwrap();
    counter.reset_count().unwrap();
    workload();
    assert_eq!(counter.stat().unwrap().event_count, 0);
}
