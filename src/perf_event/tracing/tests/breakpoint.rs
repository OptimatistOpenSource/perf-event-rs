use crate::sampling::record::{Record, RecordBody};
use crate::tracing::tests::{gen_builder, gen_cfg};
use crate::{BreakpointEvent, BreakpointLen, BreakpointType, Event};

fn test<F>(ev: &Event, workload: &mut F, addr: u64)
where
    F: FnMut(),
{
    test_next_record(ev, workload, addr);
    test_stat(ev, workload);
}

fn test_next_record<F>(ev: &Event, workload: &mut F, addr: u64)
where
    F: FnMut(),
{
    let builder = gen_builder(1 + 512);
    let mut tracer = builder.build_tracing(&gen_cfg(ev)).unwrap();

    tracer.enable().unwrap();
    workload();
    tracer.disable().unwrap();

    let mut sample_count = 0;
    for Record { body, .. } in tracer.iter() {
        if let RecordBody::Sample(body) = body {
            sample_count += 1;
            assert_eq!(body.addr.unwrap(), addr);
        }
    }
    assert!(sample_count > 0);
}

fn test_stat<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    let builder = gen_builder(1 + 512);
    let mut tracer = builder.build_tracing(&gen_cfg(ev)).unwrap();

    tracer.enable().unwrap();
    workload();
    tracer.disable().unwrap();

    let stat = tracer.stat().unwrap();
    assert!(stat.event_count > 0);
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
}

#[test]
fn test_bp_rw() {
    let mut a = 0;
    let a_addr = &a as *const _ as _;

    let bp_type = BreakpointType::Rw {
        addr: a_addr,
        len: BreakpointLen::Len1,
    };
    let ev = BreakpointEvent::new(bp_type);

    let mut workload = || {
        for i in 0..100000 {
            a = i;
            std::hint::black_box(a);
        }
    };

    test(&Event::from(ev), &mut workload, a_addr);
}
