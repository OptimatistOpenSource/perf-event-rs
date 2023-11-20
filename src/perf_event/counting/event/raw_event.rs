use crate::perf_event::counting::{Event, Inner};

pub struct RawEvent {
    config: u64,
}

impl RawEvent {
    /// # Safety
    /// The `config` argument must be valid for counting mode.
    pub unsafe fn new(config: u64) -> Self {
        RawEvent { config }
    }

    pub fn into_u64(self) -> u64 {
        self.config
    }
}

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        Self(Inner::Raw(value))
    }
}
