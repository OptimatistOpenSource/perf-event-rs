use crate::sampling::record::{Record, RecordBody};
use crate::test::{cpu_workload, read_file};
use crate::tracing::tests::{gen_builder, gen_cfg};
use crate::{Event, TracepointEvent};
use std::str::FromStr;

fn test<F>(ev: &Event, workload: &mut F)
where
    F: FnMut(),
{
    test_next_record(ev, workload);
    test_stat(ev, workload);
}

fn test_next_record<F>(ev: &Event, workload: &mut F)
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
            assert!(body.addr.is_some());
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
fn test_sched_switch() {
    let id = {
        let path = "/sys/kernel/debug/tracing/events/sched/sched_switch/id";
        let string = read_file(path).replace('\n', "");
        u64::from_str(string.as_str()).unwrap()
    };
    let ev = TracepointEvent::new(id);

    let mut workload = cpu_workload;

    test(&Event::from(ev), &mut workload);
}
