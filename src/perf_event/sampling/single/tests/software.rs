use crate::sampling::single::tests::test_single;
use crate::test::cpu_workload;
use crate::{Event, SoftwareEvent};

#[test]
fn test_cpu_clock() {
    let ev = SoftwareEvent::CpuClock;
    let mut workload = cpu_workload;
    test_single(&Event::from(ev), &mut workload);
}
