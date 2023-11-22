use crate::counting::{Attr, HwEvent};
use crate::{Builder, EventScope};

fn workload() {
    for _ in 0..10000000 {
        std::hint::black_box(0);
    }
}

#[test]
fn test() {
    let builder = Builder::new().calling_process().any_cpu();
    let mut group = builder.build_counting_group().unwrap();
    let cpu_cycles_event_id = group
        .add_member({
            let event = HwEvent::CpuCycles;
            let scopes = [EventScope::User, EventScope::Host];
            Attr::new(event, scopes, Default::default())
        })
        .unwrap();
    let instructions_event_id = group
        .add_member({
            let event = HwEvent::Instructions;
            let scopes = [EventScope::User, EventScope::Host];
            Attr::new(event, scopes, Default::default())
        })
        .unwrap();

    {
        let result = group.get_result().unwrap();
        let cpu_cycles = result.member_results.get(&cpu_cycles_event_id).unwrap();
        dbg!(cpu_cycles);
        assert_eq!(cpu_cycles.event_count, 0);
        let instructions = result.member_results.get(&instructions_event_id).unwrap();
        dbg!(instructions);
        assert_eq!(instructions.event_count, 0);
    };
    group.enable().unwrap();

    workload();

    group.disable().unwrap();
    let ipc = {
        let events = group.get_result().unwrap().member_results;
        let cpu_cycles = events.get(&cpu_cycles_event_id).unwrap();
        dbg!(cpu_cycles);
        assert!(cpu_cycles.event_count > 0);
        let instructions = events.get(&instructions_event_id).unwrap();
        dbg!(instructions);
        assert!(instructions.event_count > 0);

        group.enable().unwrap();
        let events = group.get_result().unwrap().member_results;
        assert!(cpu_cycles.event_count < events.get(&cpu_cycles_event_id).unwrap().event_count);
        assert!(instructions.event_count < events.get(&instructions_event_id).unwrap().event_count);

        instructions.event_count as f64 / cpu_cycles.event_count as f64
    };
    dbg!(ipc);
    assert!(ipc > 0_f64);
}
