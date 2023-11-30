pub struct CountingGuard {
    pub event_id: u64,
}

impl CountingGuard {
    pub(crate) const fn new(event_id: u64) -> Self {
        Self { event_id }
    }
}
