use crate::TracingEvent;

pub enum KprobeConfig {
    FuncAndOffset { kprobe_func: u64, probe_offset: u64 },
    KprobeAddr(u64),
}

pub struct UprobeConfig {
    pub uprobe_path: u64,
    pub probe_offset: u64,
}

pub enum DynamicPmuEvent {
    /// The type can be found in `/sys/bus/event_source/devices/*/type`
    OtherType(u32),
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
