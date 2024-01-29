use crate::counting::single::tests::test_single;
use crate::test::cpu_workload;
use crate::{Event, HardwareEvent};

#[test]
fn test_cpu_cycles() {
    let ev = HardwareEvent::CpuCycles;
    let mut workload = cpu_workload;

    test_single(&Event::from(ev), &mut workload);
}
