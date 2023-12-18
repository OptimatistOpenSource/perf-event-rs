use crate::counting::{Config, CountingGroup};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, HwEvent};

fn gen_group() -> CountingGroup {
    let builder = Builder::new().calling_process().any_cpu();
    builder.build_counting_group().unwrap()
}

fn gen_cfg(event: HwEvent) -> Config {
    let scopes = [EventScope::User, EventScope::Host];
    Config::new(event, scopes, Default::default())
}

#[test]
fn test_basic() {
    let mut group = gen_group();
    let cpu_cycles_guard = group.add_member(&gen_cfg(HwEvent::CpuCycles)).unwrap();
    let instructions_guard = group.add_member(&gen_cfg(HwEvent::Instructions)).unwrap();

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
    let mut group = gen_group();
    let cpu_cycles_guard = group.add_member(&gen_cfg(HwEvent::CpuCycles)).unwrap();
    let instructions_guard = group.add_member(&gen_cfg(HwEvent::Instructions)).unwrap();

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
    let mut group = gen_group();
    let cpu_cycles_guard = group.add_member(&gen_cfg(HwEvent::CpuCycles)).unwrap();
    let instructions_guard = group.add_member(&gen_cfg(HwEvent::Instructions)).unwrap();

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

#[test]
fn test_guard() {
    let mut group = gen_group();
    let mut cpu_cycles_guard = group.add_member(&gen_cfg(HwEvent::CpuCycles)).unwrap();
    let mut instructions_guard = group.add_member(&gen_cfg(HwEvent::Instructions)).unwrap();

    {
        let cpu_cycles = cpu_cycles_guard.result().unwrap().event_count;
        dbg!(cpu_cycles);
        assert_eq!(cpu_cycles, 0);
        let instructions = instructions_guard.result().unwrap().event_count;
        dbg!(instructions);
        assert_eq!(instructions, 0);
    };

    let group = group.enable().unwrap();
    cpu_workload();
    group.disable().unwrap();

    let rate = {
        let cpu_cycles = cpu_cycles_guard.result().unwrap().event_count;
        dbg!(cpu_cycles);
        assert!(cpu_cycles > 0);
        let instructions = instructions_guard.result().unwrap().event_count;
        dbg!(instructions);
        assert!(instructions > 0);

        instructions as f64 / cpu_cycles as f64
    };
    dbg!(rate);
    assert!(rate > 0_f64);
}
