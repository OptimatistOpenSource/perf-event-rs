use crate::counting::{Attr, SwEvent};
use crate::{Builder, EventScope};

fn workload(n: usize) {
    for _ in 0..n {
        std::hint::black_box(vec![0_u8; 10000000]);
    }
}

// TODO: need refactor
#[test]
fn test() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_clock_event_id = group
        .add_member({
            let event = SwEvent::CpuClock;
            let scopes = [EventScope::User, EventScope::Host];
            Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let page_faults_event_id = group
        .add_member({
            let event = SwEvent::PageFaults;
            let scopes = [EventScope::User, EventScope::Host];
            Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.get_result().unwrap();
        let cpu_clock = result.member_results.get(&cpu_clock_event_id).unwrap();
        dbg!(cpu_clock);
        assert_eq!(cpu_clock.event_count, 0);
        let page_faults = result.member_results.get(&page_faults_event_id).unwrap();
        dbg!(page_faults);
        assert_eq!(page_faults.event_count, 0);
    };
    group.enable().unwrap();

    workload(1);

    group.disable().unwrap();
    let rate = {
        let events = group.get_result().unwrap().member_results;
        let cpu_clock = events.get(&cpu_clock_event_id).unwrap();
        dbg!(cpu_clock);
        assert!(cpu_clock.event_count > 0);
        let page_faults = events.get(&page_faults_event_id).unwrap();
        dbg!(page_faults);
        assert!(page_faults.event_count > 0);

        // restart test
        group.enable().unwrap();
        workload(100);
        let events = group.get_result().unwrap().member_results;
        assert!(cpu_clock.event_count < events.get(&cpu_clock_event_id).unwrap().event_count);
        dbg!(events.get(&page_faults_event_id).unwrap().event_count);
        assert!(page_faults.event_count < events.get(&page_faults_event_id).unwrap().event_count);

        // reset_count test
        group.disable().unwrap();
        group.reset_count().unwrap();
        let events = group.get_result().unwrap().member_results;
        assert_eq!(events.get(&cpu_clock_event_id).unwrap().event_count, 0);
        assert_eq!(events.get(&page_faults_event_id).unwrap().event_count, 0);

        page_faults.event_count as f64 / cpu_clock.event_count as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}
