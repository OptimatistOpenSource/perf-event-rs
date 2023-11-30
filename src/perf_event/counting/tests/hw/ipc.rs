use crate::counting::Attr;
use crate::{Builder, EventScope, HwEvent};
use crate::test::cpu_workload;

#[test]
fn test_basic() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_cycles_event_id = group
        .add_member({
            let event = HwEvent::CpuCycles;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let instructions_event_id = group
        .add_member({
            let event = HwEvent::Instructions;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.result().unwrap();
        let cpu_cycles = result.member_results.get(&cpu_cycles_event_id).unwrap();
        dbg!(cpu_cycles);
        assert_eq!(cpu_cycles.event_count, 0);
        let instructions = result.member_results.get(&instructions_event_id).unwrap();
        dbg!(instructions);
        assert_eq!(instructions.event_count, 0);
    };

    group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let rate = {
        let events = group.result().unwrap().member_results;
        let cpu_cycles = events.get(&cpu_cycles_event_id).unwrap();
        dbg!(cpu_cycles);
        assert!(cpu_cycles.event_count > 0);
        let instructions = events.get(&instructions_event_id).unwrap();
        dbg!(instructions);
        assert!(instructions.event_count > 0);

        instructions.event_count as f64 / cpu_cycles.event_count as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}

#[test]
fn test_enable_disable() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_cycles_event_id = group
        .add_member({
            let event = HwEvent::CpuCycles;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let instructions_event_id = group
        .add_member({
            let event = HwEvent::Instructions;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.result().unwrap();
        let cpu_cycles = result.member_results.get(&cpu_cycles_event_id).unwrap();
        assert_eq!(cpu_cycles.event_count, 0);
        let instructions = result.member_results.get(&instructions_event_id).unwrap();
        assert_eq!(instructions.event_count, 0);
    };

    group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let events = group.result().unwrap().member_results;
    let cpu_cycles = events.get(&cpu_cycles_event_id).unwrap();
    assert!(cpu_cycles.event_count > 0);
    let instructions = events.get(&instructions_event_id).unwrap();
    assert!(instructions.event_count > 0);

    let events = group.result().unwrap().member_results;
    assert_eq!(
        events.get(&cpu_cycles_event_id).unwrap().event_count,
        cpu_cycles.event_count
    );
    assert_eq!(
        events.get(&instructions_event_id).unwrap().event_count,
        instructions.event_count
    );

    group.enable().unwrap();
    let events = group.result().unwrap().member_results;
    assert!(events.get(&cpu_cycles_event_id).unwrap().event_count > cpu_cycles.event_count);
    assert!(events.get(&instructions_event_id).unwrap().event_count > instructions.event_count);
}

#[test]
fn test_reset_count() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_cycles_event_id = group
        .add_member({
            let event = HwEvent::CpuCycles;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let instructions_event_id = group
        .add_member({
            let event = HwEvent::Instructions;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    {
        let events = group.result().unwrap().member_results;
        let cpu_cycles = events.get(&cpu_cycles_event_id).unwrap();
        assert!(cpu_cycles.event_count > 0);
        let instructions = events.get(&instructions_event_id).unwrap();
        assert!(instructions.event_count > 0);
    }

    group.reset_count().unwrap();

    {
        let events = group.result().unwrap().member_results;
        let cpu_cycles = events.get(&cpu_cycles_event_id).unwrap();
        assert_eq!(cpu_cycles.event_count, 0);
        let instructions = events.get(&instructions_event_id).unwrap();
        assert_eq!(instructions.event_count, 0);
    };
}
