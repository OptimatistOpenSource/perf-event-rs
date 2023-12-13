use crate::TracingEvent;

pub struct TracepointEvent {
    pub id: u64,
}

impl From<TracepointEvent> for TracingEvent {
    fn from(value: TracepointEvent) -> Self {
        Self::Tracepoint(value)
    }
}
