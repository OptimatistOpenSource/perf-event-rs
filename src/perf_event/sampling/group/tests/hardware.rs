use crate::sampling::group::tests::test_group;
use crate::test::cpu_workload;
use crate::{Event, HardwareEvent};

#[test]
fn test_ipc() {
    let ev_1 = HardwareEvent::Instructions;
    let ev_2 = HardwareEvent::CpuCycles;
    let mut workload = cpu_workload;

    test_group(&Event::from(ev_1), &Event::from(ev_2), &mut workload);
}
