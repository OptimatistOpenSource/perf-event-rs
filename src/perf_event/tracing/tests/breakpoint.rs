use crate::event::tracing::{BreakpointEvent, BreakpointLen, BreakpointType, TracepointEvent};
use crate::perf_event::RawAttr;
use crate::sampling::record::{Record, RecordBody};
use crate::test::{cpu_workload, read_file};
use crate::tracing::{Config, ExtraConfig};
use crate::{Builder, EventScope};

fn gen_builder(mmap_pages: usize) -> Builder {
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_cfg(bp_type: BreakpointType) -> Config {
    let event = BreakpointEvent::new(bp_type);
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.addr = true;
    let scopes = EventScope::all();
    Config::new(event, scopes, &extra_config)
}

#[test]
fn test_basic() {
    let builder = gen_builder(1 + 512);
    let mut a = 114514;
    let a_addr = &a as *const _ as _;
    let bp_type = {
        BreakpointType::Rw {
            addr: a_addr,
            len: BreakpointLen::Len1,
        }
    };
    let cfg = gen_cfg(bp_type);
    let tracing = builder.build_tracing(&cfg).unwrap();

    tracing.enable().unwrap();
    for i in 0..114514 {
        a = i;
        std::hint::black_box(a);
    }
    tracing.disable().unwrap();

    let mut sample_count = 0;
    for Record { body, .. } in tracing {
        if let RecordBody::Sample(body) = body {
            sample_count += 1;
            assert_eq!(body.addr.unwrap(), a_addr);
        }
    }
    assert!(sample_count > 0);
}
