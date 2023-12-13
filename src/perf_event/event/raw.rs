use crate::Event;

#[derive(Debug, Clone)]
pub struct RawEvent {
    config: u64,
}

impl RawEvent {
    /// # Safety
    /// The `config` argument must be valid for counting mode.
    pub const unsafe fn new(config: u64) -> Self {
        Self { config }
    }

    pub const fn into_u64(self) -> u64 {
        self.config
    }
}

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        Self::Raw(value)
    }
}
