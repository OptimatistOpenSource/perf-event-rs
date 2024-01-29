mod hardware;
mod software;

use crate::counting::{Config, CounterGroup};
use crate::{Builder, Event, EventScope};

/// rate = ev_1 / ev_2
pub fn test_group<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    test_stat(ev_1, ev_2, workload);
    test_enable_disable(ev_1, ev_2, workload);
    test_reset_count(ev_1, ev_2, workload);
    test_guard(ev_1, ev_2, workload);
}

fn gen_group() -> CounterGroup {
    let builder = Builder::new().calling_process().any_cpu();
    builder.build_counting_group().unwrap()
}

fn gen_cfg(ev: &Event) -> Config {
    let scopes = [EventScope::User, EventScope::Host];
    Config::new(ev, &scopes, &Default::default())
}

fn test_stat<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    {
        let result = group.stat().unwrap();
        let ev_1 = result.member_count(&ev_1_guard).unwrap();
        dbg!(ev_1);
        assert_eq!(ev_1, 0);
        let ev_2 = result.member_count(&ev_2_guard).unwrap();
        dbg!(ev_2);
        assert_eq!(ev_2, 0);
    };

    let mut group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    let rate = {
        let evs = group.stat().unwrap();
        let ev_1 = evs.member_count(&ev_1_guard).unwrap();
        dbg!(ev_1);
        assert!(ev_1 > 0);
        let ev_2 = evs.member_count(&ev_2_guard).unwrap();
        dbg!(ev_2);
        assert!(ev_2 > 0);

        ev_2 as f64 / ev_1 as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}

fn test_enable_disable<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    {
        let result = group.stat().unwrap();
        let ev_1 = result.member_count(&ev_1_guard).unwrap();
        assert_eq!(ev_1, 0);
        let ev_2 = result.member_count(&ev_2_guard).unwrap();
        assert_eq!(ev_2, 0);
    };

    let mut group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    let evs = group.stat().unwrap();
    let ev_1 = evs.member_count(&ev_1_guard).unwrap();
    assert!(ev_1 > 0);
    let ev_2 = evs.member_count(&ev_2_guard).unwrap();
    assert!(ev_2 > 0);

    let evs = group.stat().unwrap();
    assert_eq!(evs.member_count(&ev_1_guard).unwrap(), ev_1);
    assert_eq!(evs.member_count(&ev_2_guard).unwrap(), ev_2);

    group.enable().unwrap();
    workload();
    let evs = group.stat().unwrap();
    assert!(evs.member_count(&ev_1_guard).unwrap() > ev_1);
    assert!(evs.member_count(&ev_2_guard).unwrap() > ev_2);
}

fn test_reset_count<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    let mut group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    {
        let evs = group.stat().unwrap();
        let ev_1 = evs.member_count(&ev_1_guard).unwrap();
        assert!(ev_1 > 0);
        let ev_2 = evs.member_count(&ev_2_guard).unwrap();
        assert!(ev_2 > 0);
    }

    group.reset_count().unwrap();

    {
        let evs = group.stat().unwrap();
        let ev_1 = evs.member_count(&ev_1_guard).unwrap();
        assert_eq!(ev_1, 0);
        let ev_2 = evs.member_count(&ev_2_guard).unwrap();
        assert_eq!(ev_2, 0);
    };
}

fn test_guard<F>(ev_1: &Event, ev_2: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let mut group = gen_group();
    let mut ev_1_guard = group.add_member(&gen_cfg(ev_1)).unwrap();
    let mut ev_2_guard = group.add_member(&gen_cfg(ev_2)).unwrap();

    {
        let ev_1 = ev_1_guard.stat().unwrap().event_count;
        dbg!(ev_1);
        assert_eq!(ev_1, 0);
        let ev_2 = ev_2_guard.stat().unwrap().event_count;
        dbg!(ev_2);
        assert_eq!(ev_2, 0);
    };

    let group = group.enable().unwrap();
    workload();
    group.disable().unwrap();

    let rate = {
        let ev_1 = ev_1_guard.stat().unwrap().event_count;
        dbg!(ev_1);
        assert!(ev_1 > 0);
        let ev_2 = ev_2_guard.stat().unwrap().event_count;
        dbg!(ev_2);
        assert!(ev_2 > 0);

        ev_1 as f64 / ev_2 as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}
