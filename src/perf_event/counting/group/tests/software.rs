use crate::counting::group::tests::test_group;
use crate::test::mem_workload;
use crate::{Event, SoftwareEvent};

#[test]
fn test_page_fault_per_clock() {
    let ev_1 = SoftwareEvent::PageFaults;
    let ev_2 = SoftwareEvent::CpuClock;
    let mut workload = mem_workload;

    test_group(&Event::from(ev_1), &Event::from(ev_2), &mut workload);
}
