use crate::sampling::record::{Record, RecordBody};
use crate::test::{cpu_workload, read_file};
use crate::tracing::{Config, ExtraConfig};
use crate::{Builder, Event, EventScope, TracepointEvent};
use std::str::FromStr;

fn gen_builder(mmap_pages: usize) -> Builder {
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_cfg(id: u64) -> Config {
    let event = TracepointEvent::new(id);
    let scopes = EventScope::all();

    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.data_raw = true;

    Config::new(&Event::from(event), &scopes, &extra_config)
}

#[test]
fn test_basic() {
    let builder = gen_builder(1 + 512);
    let id = {
        let path = "/sys/kernel/debug/tracing/events/sched/sched_switch/id";
        let string = read_file(path).replace('\n', "");
        u64::from_str(string.as_str()).unwrap()
    };
    let cfg = gen_cfg(id);
    let mut tracer = builder.build_tracing(&cfg).unwrap();

    tracer.enable().unwrap();
    cpu_workload();
    tracer.disable().unwrap();

    let mut sample_count = 0;
    for Record { body, .. } in tracer.iter() {
        if let RecordBody::Sample(body) = body {
            sample_count += 1;
            assert!(body.data_raw.is_some());
        }
    }
    assert!(sample_count > 0);
}
