use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, Event, EventScope, HardwareEvent};

fn gen_builder() -> Builder {
    let mmap_pages = 1 + 512;
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_cfg(sample_stack_user: u16) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.data_stack_user = Some(sample_stack_user);

    let event = HardwareEvent::CpuCycles;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::new(&Event::from(event), &scopes, &overflow_by, &extra_config)
}

#[test]
fn test() {
    let builder = gen_builder();
    for i in 3..8 {
        let cfg = gen_cfg(2_u16.pow(i));
        let mut sampler = builder.build_sampling(&cfg).unwrap();

        sampler.enable().unwrap();
        cpu_workload();
        sampler.disable().unwrap();

        let mut sample_count = 0_usize;
        for Record { body, .. } in sampler.iter() {
            if let RecordBody::Sample(body) = body {
                assert!(body.data_stack_user.is_some());
                sample_count += 1;
            }
        }
        assert!(sample_count > 0);
    }
}