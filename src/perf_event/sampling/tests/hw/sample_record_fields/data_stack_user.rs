use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, HwEvent};

fn gen_builder() -> Builder {
    let mmap_pages = 1 + 512;
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_attr(sample_stack_user: u16) -> Attr {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.data_stack_user = Some(sample_stack_user);

    let event = HwEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Attr::new(event, scopes, overflow_by, &extra_config)
}

#[test]
fn test() {
    let builder = gen_builder();
    for i in 3..8 {
        let attr = gen_attr(2_u16.pow(i));
        let sampling = builder.build_sampling(&attr).unwrap();

        sampling.enable().unwrap();
        cpu_workload();
        sampling.disable().unwrap();

        let mut sample_count = 0_usize;
        for Record { body, .. } in sampling {
            if let RecordBody::Sample(body) = body {
                assert!(body.data_stack_user.is_some());
                sample_count += 1;
            }
        }
        assert!(sample_count > 0);
    }
}
