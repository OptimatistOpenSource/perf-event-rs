use crate::perf_event::event::tracing::TracingEvent;

#[derive(Clone, Debug)]
pub struct TracepointEvent {
    /// The content of `/sys/kernel/debug/tracing/events/*/*/id`
    pub id: u64,
}

impl TracepointEvent {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
}

impl From<TracepointEvent> for TracingEvent {
    fn from(value: TracepointEvent) -> Self {
        Self::Tracepoint(value)
    }
}
