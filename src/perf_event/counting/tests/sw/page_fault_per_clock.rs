use crate::counting::Attr;
use crate::test::mem_workload;
use crate::{Builder, EventScope, SwEvent};

#[test]
fn test_basic() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_clock_event_id = group
        .add_member({
            let event = SwEvent::CpuClock;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let page_faults_event_id = group
        .add_member({
            let event = SwEvent::PageFaults;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.result().unwrap();
        let cpu_clock = result.member_count(&cpu_clock_event_id).unwrap();
        dbg!(cpu_clock);
        assert_eq!(cpu_clock, 0);
        let page_faults = result.member_count(&page_faults_event_id).unwrap();
        dbg!(page_faults);
        assert_eq!(page_faults, 0);
    };

    group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    let rate = {
        let events = group.result().unwrap();
        let cpu_clock = events.member_count(&cpu_clock_event_id).unwrap();
        dbg!(cpu_clock);
        assert!(cpu_clock > 0);
        let page_faults = events.member_count(&page_faults_event_id).unwrap();
        dbg!(page_faults);
        assert!(page_faults > 0);

        page_faults as f64 / cpu_clock as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}

#[test]
fn test_enable_disable() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_clock_event_id = group
        .add_member({
            let event = SwEvent::CpuClock;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let page_faults_event_id = group
        .add_member({
            let event = SwEvent::PageFaults;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.result().unwrap();
        let cpu_clock = result.member_count(&cpu_clock_event_id).unwrap();
        assert_eq!(cpu_clock, 0);
        let page_faults = result.member_count(&page_faults_event_id).unwrap();
        assert_eq!(page_faults, 0);
    };

    group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    let events = group.result().unwrap();
    let cpu_clock = events.member_count(&cpu_clock_event_id).unwrap();
    assert!(cpu_clock > 0);
    let page_faults = events.member_count(&page_faults_event_id).unwrap();
    assert!(page_faults > 0);

    let events = group.result().unwrap();
    assert_eq!(events.member_count(&cpu_clock_event_id).unwrap(), cpu_clock);
    assert_eq!(
        events.member_count(&page_faults_event_id).unwrap(),
        page_faults
    );

    group.enable().unwrap();
    mem_workload();
    mem_workload();
    mem_workload();
    let events = group.result().unwrap();
    assert!(events.member_count(&cpu_clock_event_id).unwrap() > cpu_clock);
    assert!(events.member_count(&page_faults_event_id).unwrap() > page_faults);
}

#[test]
fn test_reset_count() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_clock_event_id = group
        .add_member({
            let event = SwEvent::CpuClock;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let page_faults_event_id = group
        .add_member({
            let event = SwEvent::PageFaults;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    {
        let events = group.result().unwrap();
        let cpu_clock = events.member_count(&cpu_clock_event_id).unwrap();
        assert!(cpu_clock > 0);
        let page_faults = events.member_count(&page_faults_event_id).unwrap();
        assert!(page_faults > 0);
    }

    group.reset_count().unwrap();

    {
        let events = group.result().unwrap();
        let cpu_clock = events.member_count(&cpu_clock_event_id).unwrap();
        assert_eq!(cpu_clock, 0);
        let page_faults = events.member_count(&page_faults_event_id).unwrap();
        assert_eq!(page_faults, 0);
    };
}
