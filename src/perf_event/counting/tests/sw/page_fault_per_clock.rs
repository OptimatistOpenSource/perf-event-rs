use crate::counting::Attr;
use crate::{Builder, EventScope, SwEvent};
use crate::test::mem_workload;

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
        let cpu_clock = result.member_results.get(&cpu_clock_event_id).unwrap();
        dbg!(cpu_clock);
        assert_eq!(cpu_clock.event_count, 0);
        let page_faults = result.member_results.get(&page_faults_event_id).unwrap();
        dbg!(page_faults);
        assert_eq!(page_faults.event_count, 0);
    };

    group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    let rate = {
        let events = group.result().unwrap().member_results;
        let cpu_clock = events.get(&cpu_clock_event_id).unwrap();
        dbg!(cpu_clock);
        assert!(cpu_clock.event_count > 0);
        let page_faults = events.get(&page_faults_event_id).unwrap();
        dbg!(page_faults);
        assert!(page_faults.event_count > 0);

        page_faults.event_count as f64 / cpu_clock.event_count as f64
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
        let cpu_clock = result.member_results.get(&cpu_clock_event_id).unwrap();
        assert_eq!(cpu_clock.event_count, 0);
        let page_faults = result.member_results.get(&page_faults_event_id).unwrap();
        assert_eq!(page_faults.event_count, 0);
    };

    group.enable().unwrap();
    mem_workload();
    group.disable().unwrap();

    let events = group.result().unwrap().member_results;
    let cpu_clock = events.get(&cpu_clock_event_id).unwrap();
    assert!(cpu_clock.event_count > 0);
    let page_faults = events.get(&page_faults_event_id).unwrap();
    assert!(page_faults.event_count > 0);

    let events = group.result().unwrap().member_results;
    assert_eq!(
        events.get(&cpu_clock_event_id).unwrap().event_count,
        cpu_clock.event_count
    );
    assert_eq!(
        events.get(&page_faults_event_id).unwrap().event_count,
        page_faults.event_count
    );

    group.enable().unwrap();
    mem_workload();
    mem_workload();
    mem_workload();
    let events = group.result().unwrap().member_results;
    assert!(events.get(&cpu_clock_event_id).unwrap().event_count > cpu_clock.event_count);
    assert!(events.get(&page_faults_event_id).unwrap().event_count > page_faults.event_count);
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
        let events = group.result().unwrap().member_results;
        let cpu_clock = events.get(&cpu_clock_event_id).unwrap();
        assert!(cpu_clock.event_count > 0);
        let page_faults = events.get(&page_faults_event_id).unwrap();
        assert!(page_faults.event_count > 0);
    }

    group.reset_count().unwrap();

    {
        let events = group.result().unwrap().member_results;
        let cpu_clock = events.get(&cpu_clock_event_id).unwrap();
        assert_eq!(cpu_clock.event_count, 0);
        let page_faults = events.get(&page_faults_event_id).unwrap();
        assert_eq!(page_faults.event_count, 0);
    };
}
