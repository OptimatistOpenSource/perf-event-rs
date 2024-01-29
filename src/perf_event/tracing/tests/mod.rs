mod breakpoint;
mod tracepoint;

use crate::tracing::{Config, ExtraConfig};
use crate::{Builder, Event, EventScope};

pub fn gen_builder(mmap_pages: usize) -> Builder {
    Builder::new()
        .calling_process()
        .any_cpu()
        .ring_buffer_pages(mmap_pages)
}

pub fn gen_cfg(ev: &Event) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.addr = true;
    let scopes = EventScope::all();
    Config::new(ev, &scopes, &extra_config)
}
