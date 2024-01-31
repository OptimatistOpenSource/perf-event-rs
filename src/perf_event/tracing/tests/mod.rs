mod breakpoint;
mod tracepoint;

use crate::config::{Cpu, Process};
use crate::tracing::{Config, ExtraConfig, Tracer};
use crate::{Event, EventScope};

fn gen_tracer(cfg: &Config) -> Tracer {
    let mmap_pages = 1 + 512;
    Tracer::new(&Process::Current, &Cpu::Any, mmap_pages, cfg).unwrap()
}

pub fn gen_cfg(ev: &Event) -> Config {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.addr = true;
    let scopes = EventScope::all();
    Config::extra_new(ev, &scopes, &extra_config)
}
