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
