use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Config, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, SwEvent};

fn gen_builder() -> Builder {
    let mmap_pages = 1 + 512;
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

fn gen_cfg(sample_regs_intr: u64) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.time = true;
    extra_config.sample_record_fields.abi_and_regs_intr = Some(sample_regs_intr);

    let event = SwEvent::CpuClock;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Config::new(event, scopes, overflow_by, &extra_config)
}

#[test]
fn test() {
    let builder = gen_builder();
    for i in 1..7 {
        let cfg = gen_cfg(i);
        let sampling = builder.build_sampling(&cfg).unwrap();

        sampling.enable().unwrap();
        cpu_workload();
        sampling.disable().unwrap();

        let mut sample_count = 0_usize;
        let mut last_time = 0;
        for Record { body, .. } in sampling {
            if let RecordBody::Sample(sample) = body {
                assert!(sample.time.unwrap() >= last_time);
                last_time = sample.time.unwrap();
                sample_count += 1;
            }
        }
        assert!(sample_count > 0);
    }
}
