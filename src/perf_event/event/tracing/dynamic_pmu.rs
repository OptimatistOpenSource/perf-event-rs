use crate::perf_event::event::tracing::TracingEvent;
use std::ffi::CString;

#[derive(Clone, Debug)]
pub enum KprobeConfig {
    FuncAndOffset {
        kprobe_func: CString,
        probe_offset: u64,
    },
    KprobeAddr(u64),
}

#[derive(Clone, Debug)]
pub struct UprobeConfig {
    pub uprobe_path: CString,
    pub probe_offset: u64,
}

#[derive(Clone, Debug)]
pub enum DynamicPmuEvent {
    Other {
        /// The content of `/sys/bus/event_source/devices/*/type`
        r#type: u32,
        /// See: `/sys/bus/event_source/devices/*/format/*`
        /// and `/sys/bus/event_source/devices/*/events/*`
        config: u64,
    },
    Kprobe {
        /// The content of `/sys/bus/event_source/devices/kprobe/type`
        r#type: u32,
        /// See `/sys/bus/event_source/devices/kprobe/format/retprobe`
        retprobe: bool,
        cfg: KprobeConfig,
    },
    Uprobe {
        /// The content of `/sys/bus/event_source/devices/uprobe/type`
        r#type: u32,
        /// See `/sys/bus/event_source/devices/uprobe/format/retprobe`
        retprobe: bool,
        cfg: UprobeConfig,
    },
}

impl From<DynamicPmuEvent> for TracingEvent {
    fn from(value: DynamicPmuEvent) -> Self {
        Self::DynamicPmu(value)
    }
}
