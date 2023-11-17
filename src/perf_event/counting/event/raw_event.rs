pub struct RawEvent {
    config: u64,
}

impl RawEvent {
    pub unsafe fn new(config: u64) -> Self {
        RawEvent { config }
    }
}
