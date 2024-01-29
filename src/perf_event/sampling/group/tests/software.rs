use crate::sampling::group::tests::test_group;
use crate::test::cpu_workload;
use crate::{Event, SoftwareEvent};

#[test]
fn test_cpu_clock_cpu_clock() {
    let ev_1 = SoftwareEvent::CpuClock;
    let ev_2 = SoftwareEvent::CpuClock;
    let mut workload = cpu_workload;

    test_group(&Event::from(ev_1), &Event::from(ev_2), &mut workload);
}
