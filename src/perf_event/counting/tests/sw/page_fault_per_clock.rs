use crate::counting::{Config, CounterGroup};
use crate::test::mem_workload;
use crate::{Builder, EventScope, SwEvent};

fn gen_group() -> CounterGroup {
    let builder = Builder::new().calling_process().any_cpu();
    builder.build_counting_group().unwrap()
}

fn gen_cfg(event: SwEvent) -> Config {
    let scopes = [EventScope::User, EventScope::Host];
    Config::new(event, scopes, Default::default())
}

#[test]
fn test_basic() {
    let mut group = gen_group();
    let cpu_clock_guard = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let page_faults_guard = group.add_member(&gen_cfg(SwEvent::PageFaults)).unwrap();

    {
        let result = group.result().unwrap();
        let cpu_clock = result.member_count(&cpu_clock_guard).unwrap();
        dbg!(cpu_clock);
        assert_eq!(cpu_clock, 0);
        let page_faults = result.member_count(&page_faults_guard).unwrap();
        dbg!(page_faults);
        assert_eq!(page_faults, 0);
    };

    let mut group = group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    let rate = {
        let events = group.result().unwrap();
        let cpu_clock = events.member_count(&cpu_clock_guard).unwrap();
        dbg!(cpu_clock);
        assert!(cpu_clock > 0);
        let page_faults = events.member_count(&page_faults_guard).unwrap();
        dbg!(page_faults);
        assert!(page_faults > 0);

        page_faults as f64 / cpu_clock as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}

#[test]
fn test_enable_disable() {
    let mut group = gen_group();
    let cpu_clock_guard = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let page_faults_guard = group.add_member(&gen_cfg(SwEvent::PageFaults)).unwrap();

    {
        let result = group.result().unwrap();
        let cpu_clock = result.member_count(&cpu_clock_guard).unwrap();
        assert_eq!(cpu_clock, 0);
        let page_faults = result.member_count(&page_faults_guard).unwrap();
        assert_eq!(page_faults, 0);
    };

    let mut group = group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    let events = group.result().unwrap();
    let cpu_clock = events.member_count(&cpu_clock_guard).unwrap();
    assert!(cpu_clock > 0);
    let page_faults = events.member_count(&page_faults_guard).unwrap();
    assert!(page_faults > 0);

    let events = group.result().unwrap();
    assert_eq!(events.member_count(&cpu_clock_guard).unwrap(), cpu_clock);
    assert_eq!(
        events.member_count(&page_faults_guard).unwrap(),
        page_faults
    );

    group.enable().unwrap();
    mem_workload();
    let events = group.result().unwrap();
    assert!(events.member_count(&cpu_clock_guard).unwrap() > cpu_clock);
    assert!(events.member_count(&page_faults_guard).unwrap() > page_faults);
}

#[test]
fn test_reset_count() {
    let mut group = gen_group();
    let cpu_clock_guard = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let page_faults_guard = group.add_member(&gen_cfg(SwEvent::PageFaults)).unwrap();

    let mut group = group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    {
        let events = group.result().unwrap();
        let cpu_clock = events.member_count(&cpu_clock_guard).unwrap();
        assert!(cpu_clock > 0);
        let page_faults = events.member_count(&page_faults_guard).unwrap();
        assert!(page_faults > 0);
    }

    group.reset_count().unwrap();

    {
        let events = group.result().unwrap();
        let cpu_clock = events.member_count(&cpu_clock_guard).unwrap();
        assert_eq!(cpu_clock, 0);
        let page_faults = events.member_count(&page_faults_guard).unwrap();
        assert_eq!(page_faults, 0);
    };
}

#[test]
fn test_guard() {
    let mut group = gen_group();
    let mut cpu_clock_guard = group.add_member(&gen_cfg(SwEvent::CpuClock)).unwrap();
    let mut page_faults_guard = group.add_member(&gen_cfg(SwEvent::PageFaults)).unwrap();

    {
        let cpu_clock = cpu_clock_guard.result().unwrap().event_count;
        dbg!(cpu_clock);
        assert_eq!(cpu_clock, 0);
        let page_faults = page_faults_guard.result().unwrap().event_count;
        dbg!(page_faults);
        assert_eq!(page_faults, 0);
    };

    let group = group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    let rate = {
        let cpu_clock = cpu_clock_guard.result().unwrap().event_count;
        dbg!(cpu_clock);
        assert!(cpu_clock > 0);
        let page_faults = page_faults_guard.result().unwrap().event_count;
        dbg!(page_faults);
        assert!(page_faults > 0);

        page_faults as f64 / cpu_clock as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}
