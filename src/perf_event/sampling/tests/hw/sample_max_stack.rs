use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, HwEvent};

fn gen_attr(sample_max_stack: u16) -> Attr {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_max_stack = Some(sample_max_stack);

    let event = HwEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Attr::new(event, scopes, overflow_by, &extra_config, [])
}

#[test]
fn test() {
    let mmap_pages = 1 + 512;
    let builder = Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages);
    for i in 1..7 {
        let attr = gen_attr(i);
        let sampling = builder.build_sampling(&attr).unwrap();

        sampling.enable().unwrap();
        cpu_workload();
        sampling.disable().unwrap();

        let mut sample_count = 0_usize;
        let mut last_time = 0;
        for Record { body, .. } in sampling {
            if let RecordBody::Sample(sample) = body {
                assert!(sample.time >= last_time);
                last_time = sample.time;
                sample_count += 1;
            }
        }
        assert!(sample_count > 0);
    }
}
