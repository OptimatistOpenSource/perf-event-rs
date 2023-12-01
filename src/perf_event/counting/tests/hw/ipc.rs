use crate::counting::Attr;
use crate::test::cpu_workload;
use crate::{Builder, EventScope, HwEvent};

#[test]
fn test_basic() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_cycles_guard = group
        .add_member({
            let event = HwEvent::CpuCycles;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let instructions_guard = group
        .add_member({
            let event = HwEvent::Instructions;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.result().unwrap();
        let cpu_cycles = result.member_count(&cpu_cycles_guard).unwrap();
        dbg!(cpu_cycles);
        assert_eq!(cpu_cycles, 0);
        let instructions = result.member_count(&instructions_guard).unwrap();
        dbg!(instructions);
        assert_eq!(instructions, 0);
    };

    let mut group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let rate = {
        let events = group.result().unwrap();
        let cpu_cycles = events.member_count(&cpu_cycles_guard).unwrap();
        dbg!(cpu_cycles);
        assert!(cpu_cycles > 0);
        let instructions = events.member_count(&instructions_guard).unwrap();
        dbg!(instructions);
        assert!(instructions > 0);

        instructions as f64 / cpu_cycles as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}

#[test]
fn test_enable_disable() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_cycles_guard = group
        .add_member({
            let event = HwEvent::CpuCycles;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let instructions_guard = group
        .add_member({
            let event = HwEvent::Instructions;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.result().unwrap();
        let cpu_cycles = result.member_count(&cpu_cycles_guard).unwrap();
        assert_eq!(cpu_cycles, 0);
        let instructions = result.member_count(&instructions_guard).unwrap();
        assert_eq!(instructions, 0);
    };

    let mut group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let events = group.result().unwrap();
    let cpu_cycles = events.member_count(&cpu_cycles_guard).unwrap();
    assert!(cpu_cycles > 0);
    let instructions = events.member_count(&instructions_guard).unwrap();
    assert!(instructions > 0);

    let events = group.result().unwrap();
    assert_eq!(events.member_count(&cpu_cycles_guard).unwrap(), cpu_cycles);
    assert_eq!(
        events.member_count(&instructions_guard).unwrap(),
        instructions
    );

    group.enable().unwrap();
    cpu_workload();
    let events = group.result().unwrap();
    assert!(events.member_count(&cpu_cycles_guard).unwrap() > cpu_cycles);
    assert!(events.member_count(&instructions_guard).unwrap() > instructions);
}

#[test]
fn test_reset_count() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_cycles_guard = group
        .add_member({
            let event = HwEvent::CpuCycles;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let instructions_guard = group
        .add_member({
            let event = HwEvent::Instructions;
            let scopes = [EventScope::User, EventScope::Host];
            &Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    let mut group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    {
        let events = group.result().unwrap();
        let cpu_cycles = events.member_count(&cpu_cycles_guard).unwrap();
        assert!(cpu_cycles > 0);
        let instructions = events.member_count(&instructions_guard).unwrap();
        assert!(instructions > 0);
    }

    group.reset_count().unwrap();

    {
        let events = group.result().unwrap();
        let cpu_cycles = events.member_count(&cpu_cycles_guard).unwrap();
        assert_eq!(cpu_cycles, 0);
        let instructions = events.member_count(&instructions_guard).unwrap();
        assert_eq!(instructions, 0);
    };
}
